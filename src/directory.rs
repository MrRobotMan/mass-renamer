use crate::{file::FileError, File};
use std::{
    env,
    fs::{canonicalize, read_dir},
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Default)]
pub struct Directory {
    _files: Vec<File>,
}

impl Directory {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DirectoryError> {
        let path = get_directory(path)?;
        let mut files = vec![];
        if let Some(p) = path.parent() {
            if let Ok(file) = p.try_into() {
                files.push(file)
            };
        };
        for p in read_dir(&path)? {
            if let Ok(file) = p?.path().try_into() {
                files.push(file);
            }
        }
        Ok(Self { _files: files })
    }
}

/// Get the full path of a directory falling back to the home directory
/// if nothing is provided. If the provided path is a file, the file's parent
/// is returned.
pub fn get_initial_directory<P: AsRef<Path>>(path: Option<P>) -> Result<PathBuf, DirectoryError> {
    if let Some(arg) = path {
        get_directory(arg)
    } else {
        match env::current_dir() {
            Ok(dir) => Ok(dir),
            Err(_) => {
                let d = home::home_dir();
                match d {
                    Some(path) => Ok(path),
                    None => Err(DirectoryError::NoHome),
                }
            }
        }
    }
}

/// Get the full path of a directory.
/// If the provided path is a file, the file's parent is returned.
pub fn get_directory<P: AsRef<Path>>(path: P) -> Result<PathBuf, DirectoryError> {
    let p = canonicalize(path)?;
    if p.is_file() {
        Ok(p.parent().unwrap().into())
    } else {
        Ok(p)
    }
}

#[derive(Debug, Error)]
pub enum DirectoryError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("No home directory could be found")]
    NoHome,
    #[error(transparent)]
    File(#[from] FileError),
}
