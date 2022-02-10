mod ui;
mod logic;
mod config;
mod dirs;

use ui::App;
use log::debug;
use gtk::prelude::ApplicationExtManual;

fn main() -> Result<(), i32> {
    env_logger::init();
    debug!("Logger got initialized.");
    gtk::init().unwrap();

    let app = App::new("com.molytho.sirula-next-gen");
    let result = app.run();
    if result == 0 {
        Ok(())
    } else {
        Err(result)
    }
}