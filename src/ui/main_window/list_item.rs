use once_cell::sync::OnceCell;
use gtk::prelude::ToValue;
use gtk::prelude::ImageExt;
use gtk::prelude::LabelExt;
use log::warn;
use once_cell::sync::Lazy;
use glib::{ParamSpec, ParamFlags, Value};
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

impl ObjectImpl for ListItemImpl {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpec::new_string(
                    "label",
                    "label",
                    "label",
                    None,
                    ParamFlags::READWRITE
                ),
                ParamSpec::new_int(
                    "pixel-size",
                    "pixel-size",
                    "pixel-size",
                    16,
                    512,
                    64,
                    ParamFlags::READWRITE
                ),
                ParamSpec::new_int(
                    "lines",
                    "lines",
                    "lines",
                    -1,
                    i32::MAX,
                    2,
                    ParamFlags::READWRITE
                )
            ]
        });      
        PROPERTIES.as_ref()
    }
    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "lines" => {
                let value = value.get().expect("Expected a i32 type");
                self.label.set_lines(value);
            },
            "pixel-size" => {
                let value = value.get().expect("Expected a i32 type");
                self.image.set_pixel_size(value);
            },
            "label" => {
                let value = value.get().expect("Expected a &str");
                self.label.set_label(value);
            },
            _ => {
                warn!("Unimplemented!: {} {:?}", pspec.name(), value);
            }
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "lines" => {
                self.label.lines().to_value()
            },
            "pixel-size" => {
                self.image.pixel_size().to_value()
            },
            "label" => {
                self.label.label().to_value()
            },
            _ => {
                warn!("Unimplemented!: {}", pspec.name());
                unimplemented!()
            }
        }
    }
}
impl WidgetImpl for ListItemImpl {}
impl ContainerImpl for ListItemImpl {}
impl BinImpl for ListItemImpl {}
impl ListBoxRowImpl for ListItemImpl {}