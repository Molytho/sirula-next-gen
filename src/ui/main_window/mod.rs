mod main_window_priv;
use main_window_priv::MainWindowImpl;

use glib::Object;
use gtk::{Application, Inhibit, gio, glib};
use gtk::prelude::{IsA, GtkWindowExt};

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<MainWindowImpl>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Bin, gtk::Container, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Buildable;
}
impl MainWindow {
    fn on_key_press_event(&self, event: &gtk::gdk::EventKey) -> Inhibit {
        use gtk::gdk::keys::constants::*;
        #[allow(non_upper_case_globals)]
        Inhibit(
            match event.keyval() {
                Escape => {
                    self.close();
                    true
                },
                _ => {
                    false
                }
            }
        )
    }
    fn on_search_term_changed(&self, text: glib::GString) {
        println!("{}", text);
    }

    pub fn new<T : IsA<Application>>(app: &T) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create Window")
    }
}