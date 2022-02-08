mod dirs;
mod config;
mod logic;

use std::path::Path;
use log::{debug, info};
use config::Config;
use dirs::Dirs;
use crate::logic::{Id, Controller};

fn main() -> Result<(), i32> {
    env_logger::init();
    debug!("Logger got initialized.");

    let dirs = Dirs::new("sirula").unwrap();
    let config = Config::from_path(Path::new("config.toml"))
        .expect("Could not open config file");
    debug!("{:?}", config);

    let mut controller = Controller::new(&config, &dirs);
    debug!("{:?}", controller);
    let input = read_input();
    controller.set_search_term(input);
    debug!("{:?}", controller);
    info!("Available options:");
    let mut count = 1;
    for item in controller.iter() {
        info!("{}: {}", count, item);
        count += 1;
    }
    controller.select(Id::new(0, 0))
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();

    input
}