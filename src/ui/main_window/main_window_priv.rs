use gtk::prelude::Cast;
use crate::ui::main_window::ListItem;
use gtk::prelude::ListBoxExt;
use gtk::prelude::EntryExt;
use gtk::EditableSignals;
use glib::subclass::InitializingObject;
use gtk::prelude::{InitializingWidgetExt, WidgetExt};
use gtk::subclass::widget::{WidgetClassSubclassExt, CompositeTemplate};
use gtk::subclass::prelude::{ApplicationWindowImpl, WindowImpl, BinImpl, ContainerImpl, WidgetImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, TemplateChild};
use gtk::{ApplicationWindow, Entry, ListBox, CompositeTemplate, glib};

#[derive(CompositeTemplate, Default)]
#[template(file = "main_window.ui")]
pub struct MainWindowImpl {
    #[template_child]
    pub list_box: TemplateChild<ListBox>,
    #[template_child]
    pub entry: TemplateChild<Entry>
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindowImpl {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = ApplicationWindow;

    fn class_init(class: &mut Self::Class) {
        Self::bind_template(class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindowImpl {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        obj.connect_key_press_event(Self::Type::on_key_press_event);
        self.entry.connect_changed(
            glib::clone!(@weak obj => move |e| obj.on_search_term_changed(e.text()))
        );
        let list_box = &self.list_box.get();
        self.entry.connect_activate(
            glib::clone!(@weak list_box => move |_| {
                if let Some(row) = list_box.selected_row() {
                    row.activate();
                } else if let Some(row) = list_box.row_at_index(0) {
                    row.activate();
                }
            })
        );

        self.list_box.connect_row_activated(
            glib::clone!(@weak obj => move |_, list_item| {
                let list_item = list_item.downcast_ref::<ListItem>().unwrap();
                let id = list_item.get_id();
                obj.on_selected(id);
            })
        );
    }
}
impl WidgetImpl for MainWindowImpl {}
impl ContainerImpl for MainWindowImpl {}
impl BinImpl for MainWindowImpl {}
impl WindowImpl for MainWindowImpl {}
impl ApplicationWindowImpl for MainWindowImpl {}