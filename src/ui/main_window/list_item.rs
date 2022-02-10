use glib::subclass::InitializingObject;
use gtk::prelude::InitializingWidgetExt;
use gtk::subclass::widget::{WidgetClassSubclassExt, CompositeTemplate};
use gtk::subclass::prelude::{BinImpl, ContainerImpl, WidgetImpl, ObjectImpl, ObjectSubclass, TemplateChild, ListBoxRowImpl};
use gtk::{ListBoxRow, Image, Label, CompositeTemplate, glib};

#[derive(CompositeTemplate, Default)]
#[template(file = "list_item.ui")]
pub struct ListItemImpl {
    #[template_child]
    pub image: TemplateChild<Image>,
    #[template_child]
    pub label: TemplateChild<Label>
}

#[glib::object_subclass]
impl ObjectSubclass for ListItemImpl {
    const NAME: &'static str = "ListItem";
    type Type = super::ListItem;
    type ParentType = ListBoxRow;

    fn class_init(class: &mut Self::Class) {
        Self::bind_template(class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ListItemImpl {}
impl WidgetImpl for ListItemImpl {}
impl ContainerImpl for ListItemImpl {}
impl BinImpl for ListItemImpl {}
impl ListBoxRowImpl for ListItemImpl {}