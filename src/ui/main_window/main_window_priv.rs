use glib::subclass::InitializingObject;
use gtk::prelude::InitializingWidgetExt;
use gtk::subclass::widget::{WidgetClassSubclassExt, CompositeTemplate};
use gtk::subclass::prelude::{ApplicationWindowImpl, WindowImpl, BinImpl, ContainerImpl, WidgetImpl, ObjectImpl, ObjectSubclass, TemplateChild};
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

impl ObjectImpl for MainWindowImpl {}
impl WidgetImpl for MainWindowImpl {}
impl ContainerImpl for MainWindowImpl {}
impl BinImpl for MainWindowImpl {}
impl WindowImpl for MainWindowImpl {}
impl ApplicationWindowImpl for MainWindowImpl {}