use std::{
    env,
    fs::canonicalize,
    io::{stdin, stdout, Write},
    path::PathBuf,
};

use thiserror::Error;
// pub mod gui;
// pub use gui::Renamer;
pub mod file;

use file::File;
use std::ffi::OsStr;

#[derive(Debug, Default)]
pub struct Files {
    selected: Vec<File>,
}

impl Files {
    pub fn clear(&mut self) {
        self.selected.clear()
    }

    pub fn add(&mut self, file: PathBuf) {
        if let Some(file) = File::new(file.as_path()) {
            self.selected.push(file)
        }
    }
}

/// Get the user's input as a usize.
pub fn get_input() -> usize {
    loop {
        let mut s = String::new();
        let _ = stdout().flush();
        if stdin().read_line(&mut s).is_err() {
            println!("Error reading input, try again.")
        };
        match s.trim().parse::<usize>() {
            Ok(size) => return size,
            Err(_) => println!("Enter only numbers."),
        }
    }
}

/// Get the full path of a directory falling back to the home directory
/// if nothing is provided. If the provided path is a file, the file's parent
/// is returned.
pub fn get_directory(path: Option<String>) -> Result<PathBuf, DirectoryError> {
    if let Some(arg) = path {
        let p = canonicalize(arg)?;
        if p.is_file() {
            Ok(p.parent().unwrap().into())
        } else {
            Ok(p)
        }
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

#[derive(Debug, Error)]
pub enum DirectoryError {
    #[error("{source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("No home directory could be found")]
    NoHome,
}

/// Convert a Path to a mutable string
fn generate_path_as_string(part: Option<&OsStr>) -> Option<String> {
    part.map(|s| s.to_string_lossy().into_owned())
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
