use std::rc::Rc;
use std::path::Path;
use std::process::Command;
use crate::{Id, Dirs, config::ModuleConfig};
use crate::logic::{Item, ItemModul, CacheControl};

static TEXT: &str = "Execute as command";

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
        &TEXT
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
    dirs: &'a Dirs
}
impl<'a> ConsoleModule<'a> {
    pub fn new(config: Option<ModuleConfig<'a>>, dirs: &'a Dirs, id: u16) -> Self {
        ConsoleModule { item: ConsoleItem::new(id), config, dirs }
    }
    pub fn boxed_item_module(config: Option<ModuleConfig<'a>>, dirs: &'a Dirs, id: u16) -> Box<dyn ItemModul + 'a> {
        Box::new(ConsoleModule::new(config, dirs, id))
    }
}
impl ItemModul for ConsoleModule<'_> {
    fn set_search_term(&mut self, search_term: Rc<String>) {
        self.item.command = search_term;
    }
    fn select(&self, _: Id) -> Result<(), i32> {
        debug_assert!(!self.item.command.is_empty());
        match Command::new("/usr/bin/alacritty")
            .arg("-e")
            .arg("sh")
            .arg("-c")
            .arg(self.item.command.as_ref())
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(_) => Err(-1)
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