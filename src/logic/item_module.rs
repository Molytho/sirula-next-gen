use crate::logic::{Id, Item};
use std::rc::Rc;

pub trait ItemModul : std::fmt::Debug {
    fn set_search_term(&mut self, term: Rc<String>);
    fn get_items(&self) -> Vec<&(dyn Item)>;
    fn select(&self, id: Id) -> Result<(), i32>; //This will be used as exit code
}