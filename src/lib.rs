pub mod file {
    pub mod add;
    pub mod case;
    pub mod name;
    pub mod reg;
    pub mod remove;
    pub mod replace;
    pub use add::AddOptions;
    pub use case::{Case, CaseOptions};
    pub use name::NameOptions;
    pub use reg::RegexOptions;
    pub use remove::{LeadDots, RemoveOptions};
    pub use replace::ReplaceOptions;

    pub trait Process {
        #[allow(unused_variables)]
        fn process(&self, file: &mut String) {}
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
    ///    -  7 Auto Date - To Do
    ///    -  8 Append Folder Name - To Do
    ///    -  9 Numbering - To Do
    ///    - 10 Extension - To Do
    ///
    /// # Example
    ///
    /// ```
    /// # use std::path::{Path, PathBuf};
    /// # use bulk_rename::file::{name::NameOptions, case::{Case, CaseOptions}, rename_file, Process};
    /// let file = "file";
    /// let ext = ".txt";
    /// let name = NameOptions::Fixed("new_name".to_owned());
    /// let case = CaseOptions{case: Case::Upper, snake: false, exceptions: Some(&"n")};
    /// let modes: Vec<Box<dyn Process>> = vec![Box::new(name), Box::new(case)];
    /// let new_name = rename_file(file, ext, modes);
    /// assert_eq!(new_name, "nEW_nAME.txt");
    /// ```
    pub fn rename_file(file: &str, ext: &str, options: Vec<Box<dyn Process>>) -> String {
        let mut new_name = String::from(file);
        let extension = String::from(ext);
        for opt in options {
            opt.process(&mut new_name)
        }
        new_name.push_str(&extension);
        new_name
    }

    #[cfg(test)]
    mod file_tests {
        use super::*;

        #[test]
        fn test_regex() {
            let file = "Testfile123";
            let ext = ".txt";
            let expected = String::from("TestfileABC.txt");
            let opt = RegexOptions {
                exp: "123",
                rep: "ABC",
            };
            let modes: Vec<Box<dyn Process>> = vec![Box::new(opt)];
            let result = rename_file(file, ext, modes);
            assert_eq!(result, expected)
        }

        #[test]
        fn test_name() {
            let file = "file";
            let ext = ".txt";
            let name = NameOptions::Fixed("new_name".to_owned());
            let modes: Vec<Box<dyn Process>> = vec![Box::new(name)];
            let new_name = rename_file(file, ext, modes);
            assert_eq!(new_name, String::from("new_name.txt"))
        }
    }
}
