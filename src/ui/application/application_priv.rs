use once_cell::unsync::OnceCell;
use std::rc::Rc;
use crate::Config;
use gtk::glib;
use gtk::subclass::prelude::{ApplicationImpl, ObjectImpl, ObjectSubclass, GtkApplicationImpl};

#[derive(Default)]
pub struct AppImpl {
    config: OnceCell<Rc<Config>>
}

#[glib::object_subclass]
impl ObjectSubclass for AppImpl {
    const NAME: &'static str = "App";
    type Type = super::App;
    type ParentType = gtk::Application;
}

impl AppImpl {
    pub fn init(&self, config: Rc<Config>) {
        self.config.set(config).unwrap();
    }
}
impl ObjectImpl for AppImpl {}
impl ApplicationImpl for AppImpl {
    fn activate(&self, application: &Self::Type) {
        application.build_ui(self.config.get().unwrap().get_module_config("UI").ok());
    }
}
impl GtkApplicationImpl for AppImpl {}