pub mod file {
    pub mod add;
    pub mod case;
    pub mod date;
    pub mod extension;
    pub mod folder;
    pub mod name;
    pub mod number;
    pub mod reg;
    pub mod remove;
    pub mod replace;
    pub use add::AddOptions;
    pub use case::{Case, CaseOptions};
    pub use date::{DateFormat, DateMode, DateOptions, DatePrefix, DateSuffix};
    pub use extension::ExtensionOptions;
    pub use folder::{FolderMode, FolderOptions};
    pub use name::NameOptions;
    pub use reg::RegexOptions;
    pub use remove::RemoveOptions;
    pub use replace::ReplaceOptions;
    use std::{
        ffi::OsStr,
        path::{Path, PathBuf},
    };

    pub trait Process {
        #[allow(unused_variables)]
        fn process(&self, file: &mut RenameFile) {}
    }

    pub struct RenameFile<'a> {
        stem: String,
        extension: Option<String>,
        original: &'a Path,
    }

    impl RenameFile<'_> {
        pub fn new(path: &Path) -> Option<RenameFile> {
            if path.is_dir() {
                return None;
            }
            let extension = generate_path_as_string(path.extension());
            match generate_path_as_string(path.file_stem()) {
                None => None,
                Some(stem) => Some(RenameFile {
                    stem,
                    extension,
                    original: path,
                }),
            }
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
        /// # use bulk_rename::file::{name::NameOptions, case::{Case, CaseOptions}, RenameFile, Process};
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
        match part {
            Some(s) => Some(s.to_string_lossy().into_owned()),
            None => None,
        }
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
            let name = NameOptions::Fixed("new_name".to_owned());
            let modes: Vec<Box<dyn Process>> = vec![Box::new(name)];
            let mut rename = RenameFile::new(file).unwrap();
            let new_name = rename.rename(modes);
            assert_eq!(new_name, expected)
        }
    }
}
