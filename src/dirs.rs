use log::debug;

use std::fs::{File, OpenOptions};
use std::fmt::Display;
use std::error::Error;
use xdg::BaseDirectoriesError;

#[derive(Debug)]
pub enum DirError {
    DirNotAccesible(BaseDirectoriesError),
    FileNotFound(std::io::Error),
}
impl From<std::io::Error> for DirError {
    fn from(inner: std::io::Error) -> Self {
        DirError::FileNotFound(inner)
    }
}
impl From<BaseDirectoriesError> for DirError {
    fn from(inner: BaseDirectoriesError) -> Self {
        DirError::DirNotAccesible(inner)
    }
}
impl Display for DirError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            DirError::DirNotAccesible(e) => {
                write!(fmt, "DirNotAccesible caused by {}", e)
            },
            DirError::FileNotFound(e) => {
                write!(fmt, "FileNotFounde caused by {}", e)
            }
        }
    }
}
impl Error for DirError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DirError::FileNotFound(e) => {
                Some(e)
            }
            DirError::DirNotAccesible(e) => {
                Some(e)
            }
        }
    }
}

type DirResult<T> = Result<T, DirError>;

#[derive(Debug)]
pub struct Dirs {
    xdg_dirs: xdg::BaseDirectories
}

impl Dirs {
    pub fn new(dir_name: &str) -> DirResult<Dirs> {
        debug!("Creating Dirs object with prefix {}", dir_name);
        Ok(Dirs { xdg_dirs: xdg::BaseDirectories::with_prefix(dir_name)? })
    }

    pub fn get_config_file(&self, name: &str) -> DirResult<File> {
        debug!("Try opening config file with name {}", name);
        let path = self.xdg_dirs.get_config_file(name);
        Ok(File::open(path)?)
    }

    pub fn get_cache_file(&self, name: &str) -> DirResult<File> {
        debug!("Opening cache file with name {}", name);
        let path = self.xdg_dirs.place_cache_file(name)?;
        Ok(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?)
    }
}