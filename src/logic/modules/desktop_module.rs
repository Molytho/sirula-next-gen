use log::error;
use std::rc::Rc;

use crate::logic::{Id, Item};
use crate::logic::ItemModul;
use crate::config::ModuleConfig;

use gtk::glib::GString;
use gtk::gio::{AppInfo, AppLaunchContext, Icon};
use gtk::prelude::AppInfoExt;

#[derive(Debug)]
pub struct AppInfoItem {
    info: AppInfo,
    display_name: GString,
    id: Id
}
impl AppInfoItem {
    pub fn new(mod_id: u16, item_id: u16, info: AppInfo) -> Self {
        let display_name = info.display_name();
        Self { info, display_name, id: Id::new(mod_id, item_id) }
    }
}
impl Item for Rc<AppInfoItem> {
    fn get_main_text(&self) -> &str {
        self.display_name.as_str()
    }
    fn get_sub_text(&self) -> &str {
        ""
    }
    fn get_icon(&self) -> Option<Icon> {
        self.info.icon()
    }
    fn get_id(&self) -> Id {
        self.id
    }
}

#[derive(Debug)]
pub struct DesktopModule {
    all_apps: Vec<Rc<AppInfoItem>>,
    selected_apps: Vec<Rc<AppInfoItem>>,
}
impl DesktopModule {
    pub fn new(_: Option<ModuleConfig<'_>>, id: u16) -> Self {
        let mut i = 0;
        let all_apps: Vec<Rc<AppInfoItem>> = AppInfo::all().into_iter()
            .filter(|app| app.should_show())
            .map( |app| {
                let ret = Rc::new(AppInfoItem::new(id, i, app));
                i += 1;
                ret
            }).collect();
        
        Self { all_apps: all_apps.clone(), selected_apps: all_apps }
    }
    pub fn boxed_item_module(config: Option<ModuleConfig<'_>>, id: u16) -> Box<dyn ItemModul> {
        Box::new(Self::new(config, id))
    }
}
impl ItemModul for DesktopModule {
    fn set_search_term(&mut self, term: Rc<String>) {
        self.selected_apps = self.all_apps.iter().filter(|app| {
            app.display_name.contains(term.as_str())
        }).map(|i| Rc::clone(i)).collect();
    }
    fn get_items(&self) -> Vec<&(dyn Item)> {
        self.selected_apps.iter().map( |app| 
            app as &(dyn Item)
        ).collect()
    }
    fn select(&self, id: Id) -> Result<(), i32> {
        let index = id.get_item_id();
        let app_info = &self.all_apps[index as usize].info;
        match app_info.launch::<AppLaunchContext>(&[], None).map_err(|err| {
            error!("Application failed to start {}", err);
            -1
        }) {
            Ok(_) => Ok(()),
            Err(n) => Err(n)
        }
    }
}