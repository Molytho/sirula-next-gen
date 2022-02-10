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
    }
}
impl WidgetImpl for MainWindowImpl {}
impl ContainerImpl for MainWindowImpl {}
impl BinImpl for MainWindowImpl {}
impl WindowImpl for MainWindowImpl {}
impl ApplicationWindowImpl for MainWindowImpl {}