use std::{ffi::OsStr, path::PathBuf};

use crate::renamer::Renamer;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Selected {
    selected: Vec<Renamer>,
}

impl Selected {
    pub fn clear(&mut self) {
        self.selected.clear()
    }

    pub fn _add(&mut self, file: PathBuf) {
        if let Ok(file) = Renamer::try_from(file.as_path()) {
            self.selected.push(file)
        }
    }
}

#[derive(Debug, Error)]
pub enum RenamerError {
    #[error(transparent)]
    Directory(#[from] crate::DirectoryError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Egui(#[from] eframe::Error),
}

#[derive(Debug)]
pub enum PathString {
    Valid(String),
    Invalid(String),
}
/// Convert a Path to a mutable string
pub(crate) fn generate_path_as_string(part: Option<&OsStr>) -> Option<PathString> {
    part.map(|path| match path.to_str() {
        Some(s) => PathString::Valid(s.into()),
        None => PathString::Invalid(path.to_string_lossy().into_owned()),
    })
}
