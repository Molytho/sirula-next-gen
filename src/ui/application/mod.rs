mod application_priv;
use gtk::prelude::WidgetExt;
use crate::config::ConfigProvider;
use crate::config::ModuleConfig;
use gtk::subclass::prelude::ObjectSubclassExt;
use crate::Config;
use std::rc::Rc;
use application_priv::AppImpl;

use glib::Object;
use gtk::{gio, glib};
use gtk::prelude::GtkWindowExt;
use crate::ui::main_window::MainWindow;

glib::wrapper! {
    pub struct App(ObjectSubclass<AppImpl>)
        @extends gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl App {
    fn build_ui(&self, config: Option<ModuleConfig<'_>>) {
        let width = config.get_or_default::<i32>("width", -1);
        let height = config.get_or_default::<i32>("height", -1);
        let anchor = config.get_or_default::<Vec<bool>>("anchor", vec![true, true, true, false]);
        let exclusive = config.get_or_default::<bool>("exclusive", false);

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

        window.present();
    }

    pub fn new(app_id: &str, config: Rc<Config>) -> Self {
        let obj = Object::new(&[("application-id", &app_id)]).expect("Failed to create App");
        let priv_ = AppImpl::from_instance(&obj);
        priv_.init(config);
        obj
    }
}