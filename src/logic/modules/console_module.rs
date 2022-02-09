use log::error;
use std::rc::Rc;
use std::path::Path;
use std::process::Command;
use crate::{Id, config::ModuleConfig};
use crate::logic::{Item, ItemModul, CacheControl};

static TEXT: &str = "Execute as command";
static DEFAULT_BINARY: &str = "/usr/bin/alacritty";
static DEFAULT_ARGS: [&str; 3] = ["-e", "sh", "-c"];

#[derive(Debug)]
struct ConsoleItem {
    id: Id,
    command: Rc<String>
}
impl ConsoleItem {
    fn new(mod_id: u16) -> Self {
        ConsoleItem { id: Id::new(mod_id, 0), command: Rc::new("".to_owned()) }
    }
}
impl Item for ConsoleItem {
    fn get_main_text(&self) -> &str {
        TEXT
    }
    fn get_sub_text(&self) -> &str {
        self.command.as_str()
    }
    fn get_icon_path(&self) -> Option<&Path> {
        None
    }
    fn get_id(&self) -> Id {
        self.id
    }
    fn cache_control(&self) -> CacheControl {
        CacheControl::Icon
    }
}

#[derive(Debug)]
pub struct ConsoleModule<'a> {
    item: ConsoleItem,
    config: Option<ModuleConfig<'a>>,
}
impl<'a> ConsoleModule<'a> {
    pub fn new(config: Option<ModuleConfig<'a>>, id: u16) -> Self {
        ConsoleModule { item: ConsoleItem::new(id), config }
    }
    pub fn boxed_item_module(config: Option<ModuleConfig<'a>>, id: u16) -> Box<dyn ItemModul + 'a> {
        Box::new(ConsoleModule::new(config, id))
    }
}
impl ItemModul for ConsoleModule<'_> {
    fn set_search_term(&mut self, search_term: Rc<String>) {
        self.item.command = search_term;
    }
    fn select(&self, id: Id) -> Result<(), i32> {
        assert!(!self.item.command.is_empty());
        assert!(id.get_item_id() == 0);

        let binary = self.config.as_ref().map(|config| {
            config.get_config::<String>("binary").ok()
        }).flatten().unwrap_or(DEFAULT_BINARY.to_string());
        let args = self.config.as_ref().map(|config| {
            config.get_config::<Vec<String>>("args").ok()
        }).flatten().unwrap_or(DEFAULT_ARGS.map(|str|{str.to_string()}).to_vec());

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