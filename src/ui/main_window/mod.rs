mod main_window_priv;
use main_window_priv::MainWindowImpl;
mod list_item;
use list_item::ListItemImpl;

use crate::App;
use crate::logic::Id;

use gtk::{glib, gio, gdk_pixbuf, Application, Inhibit, Widget};
use gtk::prelude::{Cast, ImageExt, IsA, GtkWindowExt, ListBoxExt};
use gtk::subclass::prelude::ObjectSubclassExt;
use glib::Object;
use gio::ListModel;
use gdk_pixbuf::Pixbuf;

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
    fn select_first_item(&self) {
        let list_box = &MainWindowImpl::from_instance(&self).list_box.get();
        if let Some(row) = list_box.row_at_index(0) {
            list_box.select_row(Some(&row));
        }
    }
    fn on_search_term_changed(&self, text: glib::GString) {
        let app = self.application().unwrap().downcast::<App>().unwrap();
        app.update_items(text);
        self.select_first_item();
    }
    fn on_selected(&self, id: Id) {
        let app = self.application().unwrap().downcast::<App>().unwrap();
        app.select_item(id);
        self.close();
    }

    pub fn new<T : IsA<Application>>(app: &T) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create Window")
    }

    pub fn register_model<P: Fn(&Object) -> Widget + 'static>(&self, model: &impl IsA<ListModel>, create_widget_func: P) {
        let priv_ = MainWindowImpl::from_instance(&self);
        let list_box = &priv_.list_box.get();
        list_box.bind_model(Some(model), create_widget_func);
        self.select_first_item();
    }
}

glib::wrapper! {
    pub struct ListItem(ObjectSubclass<ListItemImpl>)
        @extends gtk::ListBoxRow, gtk::Bin, gtk::Container, gtk::Widget,
        @implements gtk::Actionable, gtk::Buildable;
}
impl ListItem {
    pub fn new(id: Id, label: &str, pixel_size: i32, lines: i32) -> Self {
        let obj = Object::new(&[("label", &label),("pixel-size", &pixel_size),("lines", &lines)]).expect("Failed to create item");
        let priv_ = ListItemImpl::from_instance(&obj);

        priv_.id.set(id).unwrap();

        obj
    }

    pub fn set_icon(&self, buf: &Pixbuf) {
        let priv_ = ListItemImpl::from_instance(&self);
        priv_.image.set_from_pixbuf(Some(buf));
    }

    pub fn get_id(&self) -> Id {
        let priv_ = ListItemImpl::from_instance(&self);
        priv_.id.get().unwrap().clone()
    }
}