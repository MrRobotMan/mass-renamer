pub mod case_options;
pub mod name_options;
pub mod regex_options;
pub mod replace_options;

pub mod renamer {
    use super::{
        case_options::CaseOptions, name_options::NameOptions, regex_options::RegexOptions,
        replace_options::ReplaceOptions,
    };
    use std::{
        error::Error,
        path::{Path, PathBuf},
    };

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
    /// # use bulk_rename::file::{name_options::NameOptions, renamer::rename_file, case_options::{Case, CaseOptions}};
    /// let file = Path::new("/path/to/file.txt");
    /// let name = NameOptions::Fixed("new_name");
    /// let case = CaseOptions{case: Case::Upper, snake: false, exceptions: Some(&"txt")};
    /// let modes = (None, Some(name), None, Some(case), None, None, None, None, None, None);
    /// let new_name = rename_file(file, modes);
    /// assert_eq!(new_name.unwrap(), PathBuf::from("/path/to\\NEW_NAME.txt"));
    /// ```
    pub fn rename_file(
        file: &Path,
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
    ) -> Result<PathBuf, Box<dyn Error>> {
        let mut parent = if let Some(name) = file.parent() {
            name.to_owned()
        } else {
            PathBuf::from("")
        };
        let (mut new_name, ext) = {
            if let Some(opt) = modes.0 {
                opt.process(&file)?
            } else {
                if let Some(s) = file.file_name() {
                    if let Some(e) = file.extension() {
                        (
                            s.to_str().unwrap().to_string(),
                            Some(e.to_str().unwrap().to_string()),
                        )
                    } else {
                        (s.to_str().unwrap().to_string(), None)
                    }
                } else {
                    return Err("Failed to extract file.".into());
                }
            }
        };
        if let Some(opt) = modes.1 {
            opt.process(&mut new_name);
        }
        if let Some(opt) = modes.2 {
            opt.process(&mut new_name);
        };
        if let Some(opt) = modes.3 {
            opt.process(&mut new_name)
        }

        parent.push(new_name);
        if let Some(e) = ext {
            Ok(parent.with_extension(e))
        } else {
            Ok(parent)
        }
    }

    #[cfg(test)]
    mod file_tests {
        use super::*;
        use std::path::{Path, PathBuf};
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
            let file = Path::new("Testfile123.txt");
            let expected = PathBuf::from("TestfileABC.txt");
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
            let result = rename_file(file, modes);
            assert_eq!(result.unwrap(), expected)
        }

        #[test]
        fn test_name() {
            let file = Path::new("/path/to/file.txt");
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
            let new_name = rename_file(file, modes);
            assert_eq!(new_name.unwrap(), PathBuf::from("/path/to/new_name.txt"))
        }

        #[test]
        fn test_replace() {
            let file = Path::new("/path/to/fileabc.txt");
            let opt = ReplaceOptions {
                replace: "ABC",
                with: "",
                case: false,
            };
            let modes = (
                None,
                None,
                Some(opt),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            );
            let new_name = rename_file(file, modes);
            assert_eq!(new_name.unwrap(), PathBuf::from("/path/to/file.txt"))
        }
    }
}
