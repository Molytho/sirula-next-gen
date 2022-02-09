mod main_window_priv;

use main_window_priv::MainWindowImpl;
use glib::Object;
use gtk::{Application, gio, glib};

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<MainWindowImpl>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Bin, gtk::Container, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Buildable;
}
impl MainWindow {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create Window")
    }
}