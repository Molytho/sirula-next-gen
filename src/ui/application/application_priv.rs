use std::rc::Rc;
use std::cell::RefCell;
use once_cell::unsync::OnceCell;

use crate::config::ModuleConfig;
use crate::dirs::Dirs;
use crate::local_config;
use crate::logic::Controller;
use super::list_model::Model;

use gtk::glib;
use gtk::prelude::GtkWindowExt;
use gtk::subclass::prelude::{ApplicationImpl, ApplicationImplExt, ObjectImpl, ObjectSubclass, GtkApplicationImpl};

#[derive(Default)]
pub struct AppImpl {
    pub dirs: OnceCell<Rc<Dirs>>,
    pub controller: OnceCell<RefCell<Controller>>,
    pub model: OnceCell<Model>,
    pub ui_config: OnceCell<UiConfig>,
}

#[glib::object_subclass]
impl ObjectSubclass for AppImpl {
    const NAME: &'static str = "App";
    type Type = super::App;
    type ParentType = gtk::Application;
}

impl ObjectImpl for AppImpl {}
impl ApplicationImpl for AppImpl {
    fn startup(&self, application: &Self::Type) {
        self.parent_startup(application);
        application.load_css(self.dirs.get().unwrap());
    }
    fn activate(&self, application: &Self::Type) {
        self.parent_activate(application);
        
        let ui_config = self.ui_config.get().unwrap();
        let lines = ui_config.lines;
        let pixel_size = ui_config.pixel_size;
        self.model.set(Model::new(pixel_size, lines)).unwrap();
        let model = self.model.get().unwrap();
        model.update_items(
            self.controller.get().unwrap().borrow().iter()
        );

        let window = application.build_ui(self.ui_config.get().unwrap());
        window.register_model(model, model.create_widget_fn());
        window.present();
    }
}
impl GtkApplicationImpl for AppImpl {}

local_config!(UiConfig {
    width: i32 = "width" (-1),
    height: i32 = "height" (-1),
    anchor: Vec<bool> = "anchor" (vec![true, true, true, false]),
    margin: Vec<i32> = "margin" (vec![0, 0, 0, 0]),
    exclusive: bool = "exclusive" (false),
    lines: i32 = "lines" (2),
    pixel_size: i32 = "icon-size" (64)
});