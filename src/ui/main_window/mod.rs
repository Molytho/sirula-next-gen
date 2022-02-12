mod main_window_priv;
use gtk::prelude::ListBoxExt;
use gtk::Widget;
use gio::ListModel;
use gtk::prelude::Cast;
use crate::App;
use gtk::prelude::ImageExt;
use main_window_priv::MainWindowImpl;
mod list_item;
use list_item::ListItemImpl;

use glib::Object;
use gtk::{Application, Inhibit, gio, glib};
use gtk::prelude::{IsA, GtkWindowExt};
use gtk::subclass::prelude::ObjectSubclassExt;
use gtk::gdk_pixbuf::Pixbuf;

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
        let app = self.application().unwrap().downcast::<App>().unwrap();
        app.update_items(text);
    }

    pub fn new<T : IsA<Application>>(app: &T) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create Window")
    }

    pub fn register_model<P: Fn(&Object) -> Widget + 'static>(&self, model: &impl IsA<ListModel>, create_widget_func: P) {
        let priv_ = MainWindowImpl::from_instance(&self);
        let list_box = &priv_.list_box.get();
        list_box.bind_model(Some(model), create_widget_func);
    }
}

glib::wrapper! {
    pub struct ListItem(ObjectSubclass<ListItemImpl>)
        @extends gtk::ListBoxRow, gtk::Bin, gtk::Container, gtk::Widget,
        @implements gtk::Actionable, gtk::Buildable;
}
impl ListItem {
    pub fn new(label: &str, pixel_size: i32, lines: i32) -> Self {
        Object::new(&[("label", &label),("pixel-size", &pixel_size),("lines", &lines)]).expect("Failed to create item")
    }

    pub fn set_icon(&self, buf: &Pixbuf) {
        let priv_ = ListItemImpl::from_instance(&self);
        priv_.image.set_from_pixbuf(Some(buf));
    }
}