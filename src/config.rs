use std::fmt::Debug;
use log::{debug, info, warn};

use crate::Dirs;
use crate::dirs::DirError;
use core::fmt::Display;
use std::borrow::Borrow;
use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use toml::Value;
use toml::value::Table;
use serde::de::Deserialize;

static DEFAULT_CONFIG: &str = "config.toml";

#[derive(Debug)]
pub enum ConfigErrorType {
    IOError,
    FormatError,
    NotFound,
    WrongType
}

#[derive(Debug)]
pub struct ConfigError {
    r#type: ConfigErrorType,
    inner: Option<Box<dyn Error>>
}
impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if let Some(e) = &self.inner {
            let e: &(dyn Error) = e.borrow();
            write!(f, "{:?} caused by {}", self.r#type, e)
        } else {
            write!(f, "{:?}", self.r#type)
        }
    }
}
impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Some(e) = &self.inner {
            Some(e.borrow())
        } else {
            None
        }
    }
}
impl ConfigError {
    fn new(r#type: ConfigErrorType, inner: Option<Box<dyn Error>>) -> ConfigError {
        ConfigError { r#type, inner }
    }
    pub fn get_type(&self) -> &ConfigErrorType {
        &self.r#type
    }
}
impl From<std::io::Error> for ConfigError {
    fn from(inner: std::io::Error) -> Self {
        ConfigError::new(ConfigErrorType::IOError, Some(Box::new(inner)))
    }
}
impl From<DirError> for ConfigError {
    fn from(inner: DirError) -> Self {
        ConfigError::new(ConfigErrorType::IOError, Some(Box::new(inner)))
    }
}
impl From<toml::de::Error> for ConfigError {
    fn from(inner: toml::de::Error) -> Self {
        ConfigError::new(ConfigErrorType::FormatError, Some(Box::new(inner)))
    }
}
impl From<ConfigErrorType> for ConfigError {
    fn from(r#type: ConfigErrorType) -> Self {
        ConfigError::new(r#type, None)
    }
}

type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug)]
pub struct Config {
    config: Table
}

impl Config {
    fn from_file(mut file: File) -> ConfigResult<Config> {
        debug!("Opening config file: {:?}", file);
        let mut config_toml = String::new();
        file.read_to_string(&mut config_toml)?;
        debug!("File content:\n {}", config_toml);

        let table = config_toml.parse::<Value>()?;
        debug_assert!(table.is_table());
        Ok(Config { config: table.try_into()? })
    }

    pub fn new(dirs: &Dirs) -> ConfigResult<Config> {
        let config_file = dirs.get_config_file(DEFAULT_CONFIG)?;

        Config::from_file(config_file)
    }

    pub fn from_path(path: &Path) -> ConfigResult<Config> {
        Config::from_file(File::open(path)?)
    }

    pub fn get_module_config(&self, name: &str) -> ConfigResult<ModuleConfig<'_>> {
        if self.config.contains_key(name) {
            let value = &self.config[name];
            if value.is_table() {
                debug!("Created module_config object for {}", name);
                Ok(ModuleConfig { values: value.as_table().expect("Toml library broke assumption") })
            } else {
                warn!("Config file has wrong format:\nExpected section for name {}", name);
                Err(ConfigErrorType::WrongType.into())
            }
        } else {
            info!("No configuration for module {} found", name);
            Err(ConfigErrorType::NotFound.into())
        }
    }
}

#[derive(Debug)]
pub struct ModuleConfig<'a> {
    values: &'a Table
}

impl <'a> ModuleConfig<'a> {
    pub fn get_config<'de, T : Deserialize<'de> + Debug>(&self, name: &str) -> ConfigResult<T> {
        if self.values.contains_key(name) {
            match self.values[name].clone().try_into() {
                Ok(value) => {
                    debug!("Found value {:?} for option {}", value, name);
                    Ok(value)
                }
                Err(_) => {
                    warn!("Value had wrong format: {}", name);
                    Err(ConfigErrorType::WrongType.into())
                }
            }
        } else {
            info!("Config {} not found", name);
            Err(ConfigErrorType::NotFound.into())
        }
    }
}