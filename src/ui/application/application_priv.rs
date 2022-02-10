use gtk::subclass::prelude::ApplicationImplExt;
use crate::Dirs;
use gtk::prelude::GtkWindowExt;
use std::borrow::Borrow;
use crate::ui::main_window::MainWindow;
use once_cell::unsync::OnceCell;
use std::rc::Rc;
use crate::Config;
use gtk::glib;
use gtk::subclass::prelude::{ApplicationImpl, ObjectImpl, ObjectSubclass, GtkApplicationImpl};

#[derive(Default)]
pub struct AppImpl {
    dirs: OnceCell<Rc<Dirs>>,
    config: OnceCell<Rc<Config>>,
    window: OnceCell<MainWindow>
}

#[glib::object_subclass]
impl ObjectSubclass for AppImpl {
    const NAME: &'static str = "App";
    type Type = super::App;
    type ParentType = gtk::Application;
}

impl AppImpl {
    pub fn init(&self, config: Rc<Config>, dirs: Rc<Dirs>) {
        self.config.set(config).unwrap();
        self.dirs.set(dirs).unwrap();
    }
}
impl ObjectImpl for AppImpl {}
impl ApplicationImpl for AppImpl {
    fn startup(&self, application: &Self::Type) {
        self.parent_startup(application);
        //TODO: Config and controller code here

        application.load_css(self.dirs.get().unwrap());
    }
    fn activate(&self, application: &Self::Type) {
        self.parent_activate(application);
        self.window.set(
            application.build_ui(self.config.get().unwrap().get_module_config("UI").ok())
        ).unwrap();
        self.window.get().unwrap().present();
    }
}
impl GtkApplicationImpl for AppImpl {}