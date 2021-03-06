mod list_model_priv;
use list_model_priv::{ModelImpl, ItemDataImpl};

use std::collections::VecDeque;
use crate::logic::{CacheControl, Item, Id};
use crate::ui::main_window::ListItem;
use gtk::{gio, glib, gdk_pixbuf, Widget, IconLookupFlags, IconTheme};
use gtk::prelude::{Cast, ObjectExt, ListModelExt, IconThemeExt};
use gtk::subclass::prelude::ObjectSubclassExt;
use glib::Object;
use gio::Icon;
use gdk_pixbuf::Pixbuf;


glib::wrapper! {
    pub struct Model(ObjectSubclass<ModelImpl>)
        @implements gio::ListModel;
}
impl Model {
    pub fn new(pixel_size: i32, lines: i32) -> Self {
        let self_ = Object::new(&[]).expect("Failed to create Model");
        let priv_ = ModelImpl::from_instance(&self_);

        priv_.lines.set(lines).unwrap();
        priv_.pixel_size.set(pixel_size).unwrap();

        self_
    }

    pub fn create_widget_fn(&self) -> impl Fn(&Object) -> Widget {
        let priv_ = ModelImpl::from_instance(&self);
        let pixel_size = priv_.pixel_size.get().unwrap().clone();
        let lines = priv_.lines.get().unwrap().clone();

        move |obj| Self::create_widget(obj, pixel_size, lines)
    }
    fn create_widget(obj: &Object, pixel_size: i32, lines: i32) -> Widget {
        let data = obj.downcast_ref::<ItemData>().unwrap();
        let data_priv = ItemDataImpl::from_instance(&data);

        let id = data_priv.id.get().unwrap();
        let text = data_priv.text.borrow();
        let widget = ListItem::new(id.clone(), text.as_str(), pixel_size, lines);
        if let Some(icon) = &*data_priv.icon.borrow() {
            widget.set_icon(icon);
        }
        *data_priv.widget.borrow_mut() = Some(widget.downgrade());

        widget.upcast()
    }

    pub fn update_items(&self, iter: std::vec::IntoIter<&(dyn Item)>) {
        let mut change_queue = VecDeque::<(u32, u32, u32)>::new();
        {
            let priv_ = ModelImpl::from_instance(&self);
            let mut item_vec = priv_.items.borrow_mut();
            let mut data_map = priv_.data_items.borrow_mut();
            let items: Vec<&(dyn Item)> = iter.collect();

            let mut index = 0;
            let mut add = 0;
            for i in 0..items.len() {
                let item = items[i];
                let id = item.get_id();

                let mut found = false;
                let old_index = index;
                //Search old occurance of item
                for i in index..item_vec.len() {
                    if id == item_vec[i] { 
                        index = i;
                        found = true;
                        break;
                    }
                };

                if !found {
                    // If not found its a new element
                    add += 1;
                } else if old_index == index {
                    // If found at current index submit transaction
                    if add != 0 {
                        change_queue.push_back(((i - add) as u32, 0, add as u32));
                        add = 0;
                    }
                    index += 1;
                } else {
                    // Found at a later position. Need to remove elements
                    let rem = index - old_index;
                    change_queue.push_back(((i - add) as u32, rem as u32, add as u32));
                    add = 0;
                    index += 1;
                }

                // Now update the item
                if let Some(data) = data_map.get(&id) {
                    // cached instance found: Update data and widget
                    let data_priv = ItemDataImpl::from_instance(&data);
                    match item.cache_control() {
                        CacheControl::Both => {}
                        CacheControl::Text => {
                            let icon_size = priv_.pixel_size.get().unwrap().clone();
                            data.update_icon(data_priv, item.get_icon(), icon_size, found);
                        }
                        CacheControl::Icon => {
                            let text = Self::item_format_text(item);
                            data.update_text(data_priv, text, found);
                        }
                        CacheControl::None => {
                            //TODO: Maybe: Use bitfield here
                            let text = Self::item_format_text(item);
                            data.update_text(data_priv, text, found);
                            let icon_size = priv_.pixel_size.get().unwrap().clone();
                            data.update_icon(data_priv, item.get_icon(), icon_size, found);
                        }
                    }
                }
                else {
                    // new element: Create and insert a new item
                    let icon_size = priv_.pixel_size.get().unwrap().clone();
                    let text = Self::item_format_text(item);
                    let icon = item.get_icon();
                    data_map.insert(
                        id,
                        (id, text, icon, icon_size).into()
                    );
                }
            }
            // We haven't found them so they are deleted
            let rem = item_vec.len() - index;
            // And a transaction at the end
            if add != 0 || rem != 0 {
                change_queue.push_back(((items.len() - add) as u32, rem as u32, add as u32));
            }

            *item_vec = items.into_iter().map(|item| item.get_id()).collect();
        }

        // Send the change events at the end because we need to adjust the data first.
        // This has to be outside of the scope of item_vec and data_map
        while let Some((pos, rem, add)) = change_queue.pop_front() {
            self.items_changed(pos, rem, add);
        }

    }
    fn item_format_text(item: &dyn Item) -> String {
        format!("{}: {}", item.get_main_text(), item.get_sub_text())
    }
}


glib::wrapper! {
    pub struct ItemData(ObjectSubclass<ItemDataImpl>);
}
impl ItemData {
    pub fn new(id: Id, text: String, icon: Option<Pixbuf>) -> Self {
        let obj = Object::new(&[]).expect("Failed to create item");
        let priv_ = ItemDataImpl::from_instance(&obj);

        *priv_.text.borrow_mut() = text;
        *priv_.icon.borrow_mut() = icon;
        priv_.id.set(id).unwrap();

        obj
    }
    fn load_icon(icon: Option<Icon>, icon_size: i32) -> Option<Pixbuf> {
        icon.map(|icon| {
            let icon_theme = IconTheme::default().unwrap(); //TODO: Error handling
            let lookup = icon_theme.lookup_by_gicon(&icon, icon_size, IconLookupFlags::FORCE_SIZE);

            lookup.map(|icon_info| {
                icon_info.load_icon().ok()
            }).flatten()
        }).flatten()
    }
    pub fn update_icon(&self, priv_: &ItemDataImpl, icon: Option<Icon>, icon_size: i32, update_widget: bool) {
        let mut icon_ref = priv_.icon.borrow_mut();
        *icon_ref = Self::load_icon(icon, icon_size);
        if update_widget {
            let ui = priv_.widget.borrow();
            let ui = ui.as_ref().unwrap().upgrade().unwrap();
            ui.set_property("label", &*icon_ref).unwrap();
        }
    }
    pub fn update_text(&self, priv_: &ItemDataImpl, text: String, update_widget: bool) {
        let mut text_ref = priv_.text.borrow_mut();
        *text_ref = text;
        if update_widget {
            let ui = priv_.widget.borrow();
            let ui = ui.as_ref().unwrap().upgrade().unwrap();
            ui.set_property("label", &*text_ref).unwrap();
        }
    }
}
impl From<(Id, String, Option<Icon>, i32)> for ItemData {
    fn from(input: (Id, String, Option<Icon>, i32)) -> Self {
        let (id, text, icon, icon_size) = input;
        let icon = Self::load_icon(icon, icon_size);
        Self::new(id, text, icon)
    }
}