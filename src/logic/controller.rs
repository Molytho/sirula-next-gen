use crate::config::Config;
use crate::dirs::Dirs;
use crate::logic::{Id, Item, ItemModul};
use std::rc::Rc;

use super::modules::{ConsoleModule, DesktopModule};

#[derive(Debug)]
pub struct Controller {
    item_modules: Vec<Box<dyn ItemModul>>,
}

impl Controller
{
    pub fn new(config: Option<Config>, _dirs: Rc<Dirs>) -> Self {
        let mut item_modules = Vec::<Box<dyn ItemModul>>::new();

        //TODO Proper module creation
        item_modules.push(
            DesktopModule::boxed_item_module(
                config.as_ref().map(|c| c.get_module_config("Desktop").ok()).flatten(),
                0
            )
        );
        item_modules.push(
            ConsoleModule::boxed_item_module(
                config.as_ref().map(|c| c.get_module_config("Console").ok()).flatten(),
                1
            )
        );

        Controller { item_modules }
    }
    pub fn select(&self, id: Id) -> Result<(), i32> {
        let index: usize = id.get_mod_id().into();
        let module = &self.item_modules[index];
        module.select(id)
    }
    pub fn set_search_term(&mut self, search_term: String) {
        let search_term = Rc::new(search_term);
        for module in &mut self.item_modules {
            module.set_search_term(Rc::clone(&search_term));
        }
    }

    pub fn iter(&self) -> std::vec::IntoIter<&(dyn Item)> {
        let mut vec = Vec::<&(dyn Item)>::new();
        for module in &self.item_modules {
            let mut vec_tmp = module.get_items();
            vec.append(&mut vec_tmp);
        }
        vec.into_iter()
    }
}