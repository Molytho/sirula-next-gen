mod application_priv;
use gtk::prelude::CssProviderExt;
use gtk::CssProvider;
use crate::Dirs;
use gtk::prelude::WidgetExt;
use crate::config::ConfigProvider;
use crate::config::ModuleConfig;
use gtk::subclass::prelude::ObjectSubclassExt;
use crate::Config;
use std::rc::Rc;
use application_priv::AppImpl;

use glib::Object;
use gtk::{gio, glib};
use log::error;
use crate::ui::main_window::MainWindow;

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
    fn build_ui(&self, config: Option<ModuleConfig<'_>>) -> MainWindow {
        let width = config.get_or::<i32>("width", -1);
        let height = config.get_or::<i32>("height", -1);
        let anchor = config.get_or::<Vec<bool>>("anchor", vec![true, true, true, false]);
        let margin = config.get_or::<Vec<i32>>("margin", vec![0, 0, 0, 0]);
        let exclusive = config.get_or::<bool>("exclusive", false);

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

    pub fn new(app_id: &str, config: Rc<Config>, dirs: Rc<Dirs>) -> Self {
        let obj = Object::new(&[("application-id", &app_id)]).expect("Failed to create App");
        let priv_ = AppImpl::from_instance(&obj);
        priv_.init(config, dirs);
        obj
    }
}