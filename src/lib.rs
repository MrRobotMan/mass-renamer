use std::path::{Path, PathBuf};

pub mod gui;
pub use gui::App;
mod add;
mod case;
mod date;
mod extension;
mod folder;
mod name;
mod number;
mod reg;
mod remove;
mod replace;

pub use add::AddOptions;
pub use case::{Case, CaseOptions};
pub use date::{DateFormat, DateMode, DateOptions, DatePrefix, DateSuffix, DateType};
pub use extension::ExtensionOptions;
pub use folder::{FolderMode, FolderOptions};
pub use name::NameOptions;
pub use number::{NumberFormat, NumberMode, NumberingOptions};
pub use reg::RegexOptions;
pub use remove::RemoveOptions;
pub use replace::ReplaceOptions;
use std::ffi::OsStr;

#[allow(unused_variables)]
pub trait Process {
    fn process(&self, file: &mut RenameFile) {}
}

#[derive(Debug)]
pub struct RenameFile {
    stem: String,
    extension: Option<String>,
    original: PathBuf,
}

impl RenameFile {
    pub fn new(path: &Path) -> Option<Self> {
        // if !path.is_file() {
        //     return None;
        // }
        let extension = generate_path_as_string(path.extension());
        if let Some(stem) = generate_path_as_string(path.file_stem()) {
            return Some(Self {
                stem,
                extension,
                original: path.to_owned(),
            });
        }
        None
    }

    /// Tool to rename a single file.
    /// Takes the `&path` and various options (processed in order) to return a `PathBuf`
    /// used to rename the file.
    /// Options are
    ///    -  1 RegEx
    ///    -  2 Name
    ///    -  3 Replace
    ///    -  4 Case
    ///    -  5 Remove
    ///    -  6 Add
    ///    -  7 Auto Date
    ///    -  8 Append Folder Name
    ///    -  9 Numbering
    ///    - 10 Extension
    ///
    /// # Example
    ///
    /// ```
    /// # use std::path::{Path, PathBuf};
    /// # use bulk_file_renamer::file::{NameOptions, Case, CaseOptions, RenameFile, Process};
    /// let file = Path::new("file.txt");
    /// let name = NameOptions::Fixed("new_name".to_owned());
    /// let case = CaseOptions{case: Case::Upper, snake: false, exceptions: Some(&"n")};
    /// let modes: Vec<Box<dyn Process>> = vec![Box::new(name), Box::new(case)];
    /// let mut rename = RenameFile::new(file).unwrap();
    /// let new_name = rename.rename(modes);
    /// assert_eq!(new_name, PathBuf::from("nEW_nAME.txt"));
    /// ```
    pub fn rename(&mut self, options: Vec<Box<dyn Process>>) -> PathBuf {
        for opt in options {
            opt.process(self);
        }
        let mut new_name = match self.original.parent() {
            None => PathBuf::from("/"),
            Some(p) => PathBuf::from(p),
        };
        new_name.push(Path::new(&self.stem));
        match &self.extension {
            None => new_name,
            Some(e) => new_name.with_extension(e),
        }
    }
}

/// Convert a Path to a mutable string
fn generate_path_as_string(part: Option<&OsStr>) -> Option<String> {
    part.map(|s| s.to_string_lossy().into_owned())
}

#[cfg(test)]
mod file_tests {
    use super::*;

    #[test]
    fn test_regex() {
        let file = Path::new("Testfile123.txt");
        let expected = PathBuf::from("TestfileABC.txt");
        let opt = RegexOptions {
            exp: "123",
            rep: "ABC",
            extension: false,
        };
        let modes: Vec<Box<dyn Process>> = vec![Box::new(opt)];
        let mut rename = RenameFile::new(file).unwrap();
        let result = rename.rename(modes);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_name() {
        let file = Path::new("file.txt");
        let expected = PathBuf::from("new_name.txt");
        let name = NameOptions::Fixed("new_name");
        let modes: Vec<Box<dyn Process>> = vec![Box::new(name)];
        let mut rename = RenameFile::new(file).unwrap();
        let new_name = rename.rename(modes);
        assert_eq!(new_name, expected)
    }
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
