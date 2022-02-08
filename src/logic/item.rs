use std::path::Path;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum CacheControl {
    Icon,
    Text,
    Both
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Id {
    mod_id: u16,
    item_id: u16
}
impl Id {
    pub fn new(mod_id: u16, item_id: u16) -> Self {
        Id { mod_id, item_id }
    }
    pub fn get_mod_id(&self) -> u16 {
        self.mod_id
    }
    pub fn get_item_id(&self) -> u16 {
        self.item_id
    }
}
impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}:{}", self.mod_id, self.item_id)
    }
}

pub trait Item : std::fmt::Debug {
    fn get_main_text(&self) -> &str;
    fn get_sub_text(&self) -> &str;
    fn get_icon_path(&self) -> Option<&Path>;
    fn get_id(&self) -> Id;
    fn cache_control(&self) -> CacheControl {
        CacheControl::Both
    }
    //TODO: Digest
}

impl<'a> Display for dyn Item + 'a {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}, {}: {}", self.get_main_text(), self.get_sub_text(), self.get_id())
    }
}