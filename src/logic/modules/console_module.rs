use log::error;
use std::rc::Rc;
use std::process::Command;
use crate::local_config;
use crate::{config::ModuleConfig};
use crate::logic::{Id, Item, ItemModul, CacheControl};
use gtk::glib::Cast;
use gtk::gio::{Icon, ThemedIcon};

static TEXT: &str = "Execute as command";
static DEFAULT_BINARY: &str = "/usr/bin/alacritty";
static DEFAULT_ARGS: [&str; 3] = ["-e", "sh", "-c"];

#[derive(Debug)]
struct ConsoleItem {
    id: Id,
    command: Rc<String>,
    config: ConsoleConfig
}
impl ConsoleItem {
    fn new(mod_id: u16, config: ConsoleConfig) -> Self {
        ConsoleItem { id: Id::new(mod_id, 0), command: Rc::new("".to_owned()), config }
    }
}
impl Item for ConsoleItem {
    fn get_main_text(&self) -> &str {
        TEXT
    }
    fn get_sub_text(&self) -> &str {
        self.command.as_str()
    }
    fn get_icon(&self) -> Option<Icon> {
        self.config.icon_name.as_ref().map(|icon_name| {
            ThemedIcon::new(icon_name).upcast()
        })
    }
    fn get_id(&self) -> Id {
        self.id
    }
    fn cache_control(&self) -> CacheControl {
        CacheControl::Icon
    }
}

local_config!(ConsoleConfig {
    binary: String = "binary" (DEFAULT_BINARY.to_string()),
    args: Vec<String> = "args" (DEFAULT_ARGS.map(|str|{str.to_string()}).to_vec()),
    icon_name: Option<String> = "icon-name" (None)
});

#[derive(Debug)]
pub struct ConsoleModule {
    item: ConsoleItem
}
impl ConsoleModule {
    pub fn new(config: Option<ModuleConfig<'_>>, id: u16) -> Self {
        ConsoleModule { item: ConsoleItem::new(id, ConsoleConfig::new(config)) }
    }
    pub fn boxed_item_module(config: Option<ModuleConfig<'_>>, id: u16) -> Box<dyn ItemModul> {
        Box::new(ConsoleModule::new(config, id))
    }
}
impl ItemModul for ConsoleModule {
    fn set_search_term(&mut self, search_term: Rc<String>) {
        self.item.command = search_term;
    }
    fn select(&self, id: Id) -> Result<(), i32> {
        assert!(!self.item.command.is_empty());
        assert!(id.get_item_id() == 0);

        let binary = &self.item.config.binary;
        let args = &self.item.config.args;

        let mut command = Command::new(binary);
        for arg in args {
            command.arg(arg);
        }
        match command.arg(self.item.command.as_ref())
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{}", e);
                Err(-1)
            }
        }
    }
    fn get_items(&self) -> Vec<&(dyn Item)> {
        if !self.item.command.is_empty() {
            vec![&self.item]
        } else {
            vec![]
        }
    }
}