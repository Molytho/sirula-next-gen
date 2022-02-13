mod application_priv;
use application_priv::AppImpl;
mod list_model;

use log::error;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::config::Config;
use crate::dirs::Dirs;
use crate::logic::{Id, Controller};
use crate::ui::application::application_priv::UiConfig;
use crate::ui::main_window::MainWindow;

use gtk::{gio, glib, CssProvider};
use gtk::prelude::{CssProviderExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassExt;
use glib::{Object, GString};

static XDG_DIR_NAME: &str = "sirula-next-gen";

glib::wrapper! {
    pub struct App(ObjectSubclass<AppImpl>)
        @extends gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl App {
    fn load_css(&self, dirs: &Dirs) {
        let path = dirs.get_config_file_path("style.css");
        let css_provider = CssProvider::new();
        if let Err(err) = css_provider.load_from_path(path.to_str().unwrap()) {
            error!("{}", err);
            return;
        }
        gtk::StyleContext::add_provider_for_screen(
            &gtk::gdk::Screen::default().expect("Error initializing gtk css provider."),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );  
    }
    fn build_ui(&self, config: &UiConfig) -> MainWindow {
        let width = config.width;
        let height = config.height;
        let anchor = &config.anchor;
        let margin = &config.margin;
        let exclusive = config.exclusive;

        let window = MainWindow::new(self);
        window.set_size_request(width, height);

        gtk_layer_shell::init_for_window(&window);
        gtk_layer_shell::set_keyboard_interactivity(&window, true);
        gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);
        if exclusive {
            gtk_layer_shell::auto_exclusive_zone_enable(&window)
        }
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, anchor[0]);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, anchor[1]);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, anchor[2]);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, anchor[3]);
        gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Top, margin[0]);
        gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Right, margin[1]);
        gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Bottom, margin[2]);
        gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Left, margin[3]);

        window
    }

    pub fn update_items(&self, search_term: GString) {
        let priv_ = AppImpl::from_instance(self);
        let mut controller = priv_.controller.get().unwrap().borrow_mut();
        let model = priv_.model.get().unwrap();

        controller.set_search_term(search_term.to_string());
        let iter = controller.iter();

        model.update_items(iter);
    }

    pub fn select_item(&self, id: Id) -> i32 {
        let priv_ = AppImpl::from_instance(self);
        let controller = priv_.controller.get().unwrap().borrow_mut();

        match controller.select(id) {
            Ok(_) => 0,
            Err(i) => i
        }
    }

    pub fn new(app_id: &str) -> Self {
        let obj = Object::new(&[("application-id", &app_id)]).expect("Failed to create App");
        let self_priv = AppImpl::from_instance(&obj);

        let dirs = Rc::new(Dirs::new(XDG_DIR_NAME).unwrap());
        self_priv.dirs.set(Rc::clone(&dirs)).unwrap();

        let config = Config::new(dirs.borrow()).ok();
        self_priv.ui_config.set(
            UiConfig::new(
                config.as_ref().map(|config| config.get_module_config("UI").ok()).flatten()
            )
        ).unwrap();

        let controller = Controller::new(config, dirs);
        self_priv.controller.set(RefCell::new(controller)).unwrap();

        obj
    }
}