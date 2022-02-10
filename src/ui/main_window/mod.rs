mod main_window_priv;
use gtk::prelude::ImageExt;
use gtk::prelude::LabelExt;
use main_window_priv::MainWindowImpl;
mod list_item;
use list_item::ListItemImpl;

use glib::Object;
use gtk::{Application, Inhibit, gio, glib};
use gtk::prelude::{IsA, GtkWindowExt};
use gtk::prelude::ContainerExt;
use gtk::subclass::prelude::ObjectSubclassExt;

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
        let priv_ = MainWindowImpl::from_instance(&self);
        let item = ListItem::new();
        ListItemImpl::from_instance(&item).label.set_label(text.as_str());
        ListItemImpl::from_instance(&item).label.set_lines(2);
        ListItemImpl::from_instance(&item).image.set_pixel_size(64);
        ListItemImpl::from_instance(&item).image.set_from_icon_name(Some("com.visualstudio.code.oss"), gtk::IconSize::Menu);
        priv_.list_box.add(&item);
    }

    pub fn new<T : IsA<Application>>(app: &T) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create Window")
    }
}

glib::wrapper! {
    pub struct ListItem(ObjectSubclass<ListItemImpl>)
        @extends gtk::ListBoxRow, gtk::Bin, gtk::Container, gtk::Widget,
        @implements gtk::Actionable, gtk::Buildable;
}
impl ListItem {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create item")
    }
}