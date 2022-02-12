use crate::ui::main_window::ListItem;
use crate::ui::application::list_model::ItemData;
use std::cell::RefCell;
use gio::{ListModel, Icon};
use gtk::prelude::Cast;
use gtk::prelude::StaticType;
use crate::logic::Id;
use std::collections::HashMap;
use gtk::{glib, gio};
use gtk::gdk_pixbuf::Pixbuf;
use glib::Object;
use glib::subclass::prelude::{ObjectImpl, ObjectSubclass};
use gtk::gio::subclass::prelude::ListModelImpl;

#[derive(Default)]
pub struct ModelImpl {
    pub data_items: RefCell<HashMap<Id, ItemData>>,
    pub items: RefCell<Vec<Id>>
}

#[glib::object_subclass]
impl ObjectSubclass for ModelImpl {
    const NAME: &'static str = "Model";
    type Type = super::Model;
    type ParentType = glib::Object;
    type Interfaces = (ListModel,);
}

impl ObjectImpl for ModelImpl {}
impl ListModelImpl for ModelImpl {
    fn item_type(&self, _: &Self::Type) -> gtk::glib::Type {
        ItemData::static_type()
    }
    fn n_items(&self, _: &Self::Type) -> u32 {
        self.items.borrow().len() as u32
    }
    fn item(&self, _: &Self::Type, i: u32) -> Option<Object> {
        let id = &self.items.borrow()[i as usize];
        let ui = self.data_items.borrow().get(&id).map(|item| item.clone().upcast::<Object>());
        debug_assert!(ui.is_some() || i >= self.items.borrow().len() as u32);
        debug_assert!(ui.is_none() || i < self.items.borrow().len() as u32);
        ui
    }
}

#[derive(Default)]
pub struct ItemDataImpl  {
    pub text: RefCell<String>,
    pub icon: RefCell<Option<Pixbuf>>,
    pub widget: RefCell<Option<ListItem>>
}
#[glib::object_subclass]
impl ObjectSubclass for ItemDataImpl {
    const NAME: &'static str = "ItemData";
    type Type = super::ItemData;
    type ParentType = glib::Object;
}
impl ObjectImpl for ItemDataImpl {}