pub mod file {
    pub mod case_options;
    pub mod name_options;
    pub mod regex_options;
    pub mod replace_options;
    use case_options::CaseOptions;
    use name_options::NameOptions;
    use regex_options::RegexOptions;
    use replace_options::ReplaceOptions;

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
    /// # use bulk_rename::file::{name_options::NameOptions, case_options::{Case, CaseOptions}, rename_file};
    /// let file = "file";
    /// let ext = ".txt";
    /// let name = NameOptions::Fixed("new_name");
    /// let case = CaseOptions{case: Case::Upper, snake: false, exceptions: Some(&"txt")};
    /// let modes = (None, Some(name), None, Some(case), None, None, None, None, None, None);
    /// let new_name = rename_file(file, ext, modes);
    /// assert_eq!(new_name.unwrap(), "NEW_NAME.txt");
    /// ```
    pub fn rename_file(
        file: &str,
        ext: &str,
        modes: (
            Option<RegexOptions>,
            Option<NameOptions>,
            Option<ReplaceOptions>,
            Option<CaseOptions>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    ) -> Option<String> {
        let mut new_name = String::from(file);
        let mut extension = String::from(ext);
        if let Some(opt) = modes.0 {
            opt.process(&mut new_name, &mut extension)
        };
        if let Some(opt) = modes.1 {
            opt.process(&mut new_name);
        }
        if let Some(opt) = modes.2 {
            opt.process(&mut new_name);
        };
        if let Some(opt) = modes.3 {
            opt.process(&mut new_name)
        };
        new_name.push_str(&extension);
        Some(new_name)
    }

    #[cfg(test)]
    mod file_tests {
        use super::*;
        /// let modes = (
        ///        None,
        ///        None,
        ///        None,
        ///        None,
        ///        None,
        ///        None,
        ///        None,
        ///        None,
        ///        None,
        ///        None,
        ///    )

        #[test]
        fn test_regex() {
            let file = "Testfile123";
            let ext = ".txt";
            let expected = String::from("TestfileABC.txt");
            let opt = RegexOptions {
                exp: "123",
                rep: "ABC",
                extension: true,
            };
            let modes = (
                Some(opt),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            );
            let result = rename_file(file, ext, modes);
            assert_eq!(result.unwrap(), expected)
        }

        #[test]
        fn test_name() {
            let file = "file";
            let ext = ".txt";
            let name = NameOptions::Fixed("new_name");
            let modes = (
                None,
                Some(name),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            );
            let new_name = rename_file(file, ext, modes);
            assert_eq!(new_name.unwrap(), (String::from("new_name.txt")))
        }
    }
}
