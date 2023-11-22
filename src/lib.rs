use std::{ffi::OsStr, path::PathBuf};

use thiserror::Error;
pub mod directory;
pub mod file;
pub mod gui;

pub use directory::Directory;
pub use file::File;

#[derive(Debug, Default)]
pub struct Selected {
    selected: Vec<File>,
}

impl Selected {
    pub fn clear(&mut self) {
        self.selected.clear()
    }

    pub fn add(&mut self, file: PathBuf) {
        if let Ok(file) = File::try_from(file.as_path()) {
            self.selected.push(file)
        }
    }
}

#[derive(Debug, Error)]
pub enum RenamerError {
    #[error(transparent)]
    Directory(#[from] directory::DirectoryError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Egui(#[from] eframe::Error),
}

#[derive(Debug)]
enum PathString {
    Valid(String),
    Invalid(String),
}
/// Convert a Path to a mutable string
fn generate_path_as_string(part: Option<&OsStr>) -> Option<PathString> {
    part.map(|path| match path.to_str() {
        Some(s) => PathString::Valid(s.into()),
        None => PathString::Invalid(path.to_string_lossy().into_owned()),
    })
}

#[cfg(test)]
pub(crate) mod tester {
    use std::{fs, panic};
    #[allow(unused_must_use)]
    pub(crate) fn run_test<T>(files: &Vec<&str>, test: T)
    where
        T: FnOnce() + panic::UnwindSafe,
    {
        for file in files {
            fs::File::create(file);
        }
        let result = panic::catch_unwind(test);
        for file in files {
            fs::remove_file(file);
        }
        assert!(result.is_ok())
    }
}
