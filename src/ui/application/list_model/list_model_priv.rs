use std::cell::RefCell;
use std::collections::HashMap;
use once_cell::unsync::OnceCell;

use crate::logic::Id;
use crate::ui::application::glib::WeakRef;
use crate::ui::application::list_model::ItemData;
use crate::ui::main_window::ListItem;

use gtk::{glib, gio, gdk_pixbuf};
use gtk::prelude::{Cast, StaticType};
use glib::Object;
use glib::subclass::prelude::{ObjectImpl, ObjectSubclass};
use gio::ListModel;
use gio::subclass::prelude::ListModelImpl;
use gdk_pixbuf::Pixbuf;

#[derive(Default)]
pub struct ModelImpl {
    pub data_items: RefCell<HashMap<Id, ItemData>>,
    pub items: RefCell<Vec<Id>>,
    pub pixel_size: OnceCell<i32>,
    pub lines: OnceCell<i32>
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
    pub id: OnceCell<Id>,
    pub text: RefCell<String>,
    pub icon: RefCell<Option<Pixbuf>>,
    pub widget: RefCell<Option<WeakRef<ListItem>>>
}
#[glib::object_subclass]
impl ObjectSubclass for ItemDataImpl {
    const NAME: &'static str = "ItemData";
    type Type = super::ItemData;
    type ParentType = glib::Object;
}
impl ObjectImpl for ItemDataImpl {}