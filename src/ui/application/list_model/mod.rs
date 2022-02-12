mod list_model_priv;
use gtk::prelude::IconThemeExt;
use gtk::IconTheme;
use list_model_priv::{ModelImpl, ItemDataImpl};

use gtk::prelude::ListModelExt;
use gtk::prelude::ObjectExt;
use crate::ui::main_window::ListItem;
use crate::logic::{CacheControl, Icon};
use gtk::subclass::prelude::ObjectSubclassExt;
use log::warn;
use crate::logic::Item;
use gtk::glib;
use gtk::gio;
use glib::Object;
use gtk::Widget;
use gtk::prelude::Cast;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::IconLookupFlags;

glib::wrapper! {
    pub struct Model(ObjectSubclass<ModelImpl>)
        @implements gio::ListModel;
}
impl Model {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create item")
    }

    pub fn create_widget(obj: &Object) -> Widget {
        let data: ItemData = obj.clone().downcast::<ItemData>().unwrap();
        let data_priv = ItemDataImpl::from_instance(&data);

        let text = data_priv.text.borrow();
        let widget = ListItem::new(text.as_str(), 64, 2);
        if let Some(icon) = &*data_priv.icon.borrow() {
            widget.set_icon(icon);
        }
        //TODO: Weak Reference?
        *data_priv.widget.borrow_mut() = Some(widget.clone());

        widget.upcast()
    }

    pub fn update_items(&self, iter: std::vec::IntoIter<&(dyn Item)>) {
        let mut change_stack = Vec::<(u32, u32, u32)>::new();

        {
            let mut item_vec = ModelImpl::from_instance(&self).items.borrow_mut();
            let mut data_map = ModelImpl::from_instance(&self).data_items.borrow_mut();
            let items: Vec<&(dyn Item)> = iter.collect();

            let mut index = 0;
            let mut add = 0;
            let mut rem = 0;
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
                    }
                };

                if !found {
                    // If not found its a new element
                    add += 1;
                } else if old_index == index {
                    // If found at current index submit transaction
                    if add != 0 || rem != 0 {
                        change_stack.push((index as u32, rem as u32, add as u32));
                        add = 0; rem = 0;
                    }
                    index += 1;
                } else {
                    // Found at a later position. Need to remove elements
                    rem += index - old_index;
                }

                // Now update the item
                if let Some(data) = data_map.get(&id) {
                    let data_priv = ItemDataImpl::from_instance(&data);
                    match item.cache_control() {
                        CacheControl::Both => {}
                        CacheControl::Text => {
                            warn!("CacheControl not implemented");
                        }
                        CacheControl::Icon => {
                            *data_priv.text.borrow_mut() = Self::item_format_text(item.get_main_text(), item.get_sub_text());
                            if found {
                                // This item wasn't newly added so the reference in data is valid. Update values on widget too
                                let ui = data_priv.widget.borrow();
                                let ui = ui.as_ref().unwrap();
                                let text = data_priv.text.borrow();
                                let text = text.as_str();
                                ui.set_property("label", text).unwrap();
                            }
                        }
                        CacheControl::None => {
                            warn!("CacheControl not implemented"); // ICON
                            let text = Self::item_format_text(item.get_main_text(), item.get_sub_text());
                            if found {
                                // This item wasn't newly added so the reference in data is valid. Update values on widget too
                                let ui = data_priv.widget.borrow();
                                let ui = ui.as_ref().unwrap();
                                let text = data_priv.text.borrow();
                                let text = text.as_str();
                                ui.set_property("label", text).unwrap();
                            }
                        }
                    }
                }
                else {
                    let icon: Option<Pixbuf> = match item.get_icon() {
                        Icon::Path(path) => {
                            todo!()
                        }
                        Icon::Name(name) => {
                            let icon_theme = IconTheme::default().unwrap();
                            Some(icon_theme.load_icon(name, 64, IconLookupFlags::FORCE_SIZE).unwrap().unwrap())
                        }
                        Icon::None => None
                    };
                    warn!("Icon not implemented");
                    data_map.insert(
                        id,
                        ItemData::new(Self::item_format_text(item.get_main_text(), item.get_sub_text()), icon)
                    );
                }
            }
            // We haven't found them so they are deleted
            rem += item_vec.len() - index;
            // And a transaction at the end
            if add != 0 || rem != 0 {
                change_stack.push((index as u32, rem as u32, add as u32));
            }

            *item_vec = items.into_iter().map(|item| item.get_id()).collect();
        }

        while let Some((pos, rem, add)) = change_stack.pop() {
            self.items_changed(pos, rem, add);
        }

    }
    fn item_format_text(text: &str, sub: &str) -> String {
        format!("{}: {}", text, sub)
    }
}


glib::wrapper! {
    pub struct ItemData(ObjectSubclass<ItemDataImpl>);
}
impl ItemData {
    pub fn new(text: String, icon: Option<Pixbuf>) -> Self {
        let obj = Object::new(&[]).expect("Failed to create item");
        let priv_ = ItemDataImpl::from_instance(&obj);

        *priv_.text.borrow_mut() = text;
        *priv_.icon.borrow_mut() = icon;

        obj
    }
}