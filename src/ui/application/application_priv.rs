use std::cell::RefCell;
use crate::config::{Config, ModuleConfig};
use crate::dirs::Dirs;
use crate::local_config;
use crate::logic::Controller;
use crate::ui::main_window::MainWindow;
use std::rc::Rc;
use once_cell::unsync::OnceCell;
use gtk::subclass::prelude::{ApplicationImpl, ApplicationImplExt, ObjectImpl, ObjectSubclass, GtkApplicationImpl};
use gtk::prelude::GtkWindowExt;
use gtk::glib;
use super::list_model::Model;

#[derive(Default)]
pub struct AppImpl {
    pub dirs: OnceCell<Rc<Dirs>>,
    pub config: OnceCell<Rc<Config>>,
    pub controller: OnceCell<RefCell<Controller>>,
    pub model: OnceCell<Model>,
    pub ui_config: OnceCell<UiConfig>,
    window: OnceCell<MainWindow>
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
        self.window.set(
            application.build_ui(self.ui_config.get().unwrap())
        ).unwrap();
        self.model.set(Model::new()).unwrap();
        self.window.get().unwrap().register_model(self.model.get().unwrap(), Model::create_widget);
        self.window.get().unwrap().present();
    }
}
impl GtkApplicationImpl for AppImpl {}

local_config!(UiConfig {
    width: i32 = "width" (-1),
    height: i32 = "height" (-1),
    anchor: Vec<bool> = "anchor" (vec![true, true, true, false]),
    margin: Vec<i32> = "margin" (vec![0, 0, 0, 0]),
    exclusive: bool = "exclusive" (false)
});