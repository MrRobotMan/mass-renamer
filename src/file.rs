#![allow(dead_code)] // Must disable later when project is complete.

pub mod renamer {
    use inflector::Inflector;
    use regex::Regex;
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
    /// # use bulk_rename::file::renamer::{NameOptions, rename_file};
    /// let file = Path::new("/path/to/file.txt");
    /// let name = NameOptions::Fixed("new_name");
    /// let modes = (None, Some(name), None, None, None, None, None, None, None, None);
    /// let new_name = rename_file(file, modes);
    /// assert_eq!(new_name.unwrap(), PathBuf::from("/path/to/new_name.txt"));
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
            new_name = opt.process(&new_name);
        }
        if let Some(opt) = modes.2 {
            new_name = opt.process(&new_name);
        };
        if let Some(opt) = modes.3 {
            new_name = opt.process(&new_name)
        }

        parent.push(new_name);
        if let Some(e) = ext {
            Ok(parent.with_extension(e))
        } else {
            Ok(parent)
        }
    }

    /// Options for the regex feature
    pub struct RegexOptions<'a> {
        pub exp: &'a str,
        pub rep: &'a str,
        pub extension: bool,
    }

    impl RegexOptions<'_> {
        /// Use a regular expression `Match` to find the offending text and `Replace` it with new.
        /// Check the `Include Ext.` box to include the file extension in the `Match`.
        fn process(
            &self,
            // exp: &str,
            // rep: &str,
            file: &Path,
            // extension: bool,
        ) -> Result<(String, Option<String>), Box<dyn Error>> {
            let exp = Regex::new(self.exp)?;
            let base = if self.extension {
                file.file_name()
            } else {
                file.file_stem()
            };
            let ext = file.extension();
            if let Some(s) = base {
                if let Some(s) = s.to_str() {
                    let res = exp.replace_all(s, self.rep).to_string();
                    if !self.extension {
                        if let Some(e) = ext {
                            if let Some(e) = e.to_str() {
                                return Ok((res, Some(String::from(e))));
                            } else {
                                return Err("Extension could not be converted to str.".into());
                            };
                        } else {
                            return Err("Extension does not exist.".into());
                        }
                    }
                    if let Some(new_name) = res.rsplit_once(".") {
                        return Ok((new_name.0.to_string(), Some(new_name.1.to_string())));
                    } else {
                        Ok((res, None))
                    }
                } else {
                    Err(
                        "Could not convert file to str in regex match, likely invalid unicode."
                            .into(),
                    )
                }
            } else {
                Err("Bad file in regex match.".into())
            }
        }
    }

    /// Options for uses the name feature.
    pub enum NameOptions<'a> {
        Keep,
        Remove,
        Fixed(&'a str),
        Reverse,
    }

    impl NameOptions<'_> {
        /// Using the `NameOptions` enum and the name function, return a modified string.
        /// - `Keep` - Do not change the original file name (default).
        /// - `Remove` - Completely erase the file from the selected items. This allows it to be rebuilt using components higher than (2).
        /// - `Fixed` - Specify a new file in the box for all selected items. Only really useful if you're also using the Numbering section.
        /// - `Reverse` - Reverse the name, e.g. 12345.txt becomes 54321.txt.
        fn process(&self, file: &str) -> String {
            match self {
                NameOptions::Keep => file.to_owned(),
                NameOptions::Remove => "".to_owned(),
                NameOptions::Fixed(x) => String::from(*x),
                NameOptions::Reverse => file.chars().rev().collect::<String>(),
            }
        }
    }

    /// Options for basic renaming rules.
    /// replace: text to be replaced
    /// with: new text
    /// case: true for case sensitive, false for case-insensitive
    pub struct ReplaceOptions<'a> {
        pub replace: &'a str,
        pub with: &'a str,
        pub case: bool,
    }

    impl ReplaceOptions<'_> {
        /// `Replace` the text in this field with the text in the `With` field.
        /// `Replace` can be case-sensitive using `Match Case` checkbox.
        /// Note that the `With` text is always replaced with the text as written, including any specific text case.
        fn process(&self, file: &str) -> String {
            let mut result = String::from(file);
            if self.case {
                result.replace(self.replace, self.with)
            } else {
                let start = file.to_lowercase().find(&self.replace.to_lowercase());
                let span = self.replace.len();
                match start {
                    Some(idx) => {
                        for _ in idx..(idx + span) {
                            result.remove(idx);
                        }
                        result.insert_str(idx, self.with);
                        result
                    }
                    None => result,
                }
            }
        }
    }

    /// Change the case of the file.
    /// - `Keep` - Do change the capitalization (default).
    /// - `Lower` - change all selected files to lowercase.
    /// - `Upper` - CHANGE ALL SELECTED FILES TO UPPERCASE.
    /// - `Title` - Change All Selected Files To Title Case.
    /// - `Sentence` - Change all selected files to sentence case.
    /// - `Snake` - Change_all_selected_files_to_snake_case_while_uppering_all_other_case_information_the_same.
    ///
    /// # TODO:
    /// Exceptions: You can also enter a list of "exceptions", separated by semicolons.
    /// So for example if you entered PDF;doc then any occurrence of pdf (or PDF, Pdf,
    /// etc) would be converted to upper-case, and every occurrence of DOC (or DoC)
    /// would become doc.
    pub struct CaseOptions {
        pub case: Case,
        pub snake: bool,
    }

    pub enum Case {
        Keep,
        Lower,
        Upper,
        Title,
        Sentence,
    }

    impl CaseOptions {
        fn process(&self, file: &str) -> String {
            let new_case = match self.case {
                Case::Keep => file.to_string(),
                Case::Lower => file.to_lowercase(),
                Case::Upper => file.to_uppercase(),
                Case::Title => file.to_title_case(),
                Case::Sentence => file.to_sentence_case(),
            };
            if self.snake {
                new_case.replace(" ", "_")
            } else {
                new_case
            }
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

    #[cfg(test)]
    mod regex_tests {
        use super::*;
        use std::path::Path;
        #[test]
        fn regex_test_with_extension() {
            let exp = "0123.txt";
            let rep = "ABCD.csv";
            let file = Path::new("file0123.txt");
            let opt = RegexOptions {
                exp,
                rep,
                extension: true,
            };
            let result = opt.process(file).unwrap();
            assert_eq!(
                result,
                (String::from("fileABCD"), Some(String::from("csv")))
            );
        }
        #[test]
        fn regex_test_no_extension() {
            let exp = "0123";
            let rep = "ABCD";
            let file = Path::new("file0123.txt");
            let opt = RegexOptions {
                exp,
                rep,
                extension: false,
            };
            let result = opt.process(file).unwrap();
            assert_eq!(
                result,
                (String::from("fileABCD"), Some(String::from("txt")))
            );
        }
        #[test]
        fn regex_test_no_extension_no_match() {
            let exp = "0123";
            let rep = "ABCD";
            let file = Path::new("file123.txt");
            let opt = RegexOptions {
                exp,
                rep,
                extension: false,
            };
            let result = opt.process(file).unwrap();
            assert_eq!(result, (String::from("file123"), Some(String::from("txt"))));
        }
        #[test]
        fn bad_file_with_extension() {
            let exp = "";
            let rep = "";
            let file = Path::new("");
            let opt = RegexOptions {
                exp,
                rep,
                extension: true,
            };
            let result = opt.process(file).unwrap_err();
            assert_eq!(result.to_string(), "Bad file in regex match.")
        }
        #[test]
        fn bad_file_no_extension() {
            let exp = "";
            let rep = "";
            let file = Path::new("");
            let opt = RegexOptions {
                exp,
                rep,
                extension: false,
            };
            let result = opt.process(file).unwrap_err();
            assert_eq!(result.to_string(), "Bad file in regex match.")
        }
    }

    #[cfg(test)]
    mod name_tests {
        use super::*;
        #[test]
        fn keep_name() {
            let file = String::from("file");
            let opt = NameOptions::Keep;
            let result = opt.process(&file);
            assert_eq!(result, String::from("file"));
        }
        #[test]
        fn remove_name() {
            let file = String::from("file");
            let opt = NameOptions::Remove;
            let result = opt.process(&file);
            assert_eq!(result, String::from(""));
        }
        #[test]
        fn fixed_name() {
            let file = String::from("file");
            let new_name = "renamed_file";
            let opt = NameOptions::Fixed(new_name);
            let result = opt.process(&file);
            assert_eq!(result, String::from(new_name));
        }
        #[test]
        fn reverse_name() {
            let file = String::from("file");
            let reversed = String::from("elif");
            let opt = NameOptions::Reverse;
            let result = opt.process(&file);
            assert_eq!(result, reversed);
        }
    }

    #[cfg(test)]
    mod match_tests {
        use super::*;
        #[test]
        fn no_matching_text_case_sensitive() {
            let replace = "ABC";
            let with = "123";
            let file = "fileabc";
            let case = true;
            let opt = ReplaceOptions {
                replace,
                with,
                case,
            };
            let result = opt.process(file);
            assert_eq!(result, String::from(file))
        }
        #[test]
        fn no_matching_text_case_insensitive() {
            let replace = "qrs";
            let with = "123";
            let file = "fileabc";
            let case = false;
            let opt = ReplaceOptions {
                replace,
                with,
                case,
            };
            let result = opt.process(file);
            assert_eq!(result, String::from(file))
        }
        #[test]
        fn matched_case_sensitive() {
            let replace = "abc";
            let with = "123";
            let file = "fileabc";
            let case = true;
            let opt = ReplaceOptions {
                replace,
                with,
                case,
            };
            let result = opt.process(file);
            assert_eq!(result, String::from("file123"))
        }
        #[test]
        fn matched_case_insensitive() {
            let replace = "ABC";
            let with = "123";
            let file = "fileabc";
            let case = false;
            let opt = ReplaceOptions {
                replace,
                with,
                case,
            };
            let result = opt.process(file);
            assert_eq!(result, String::from("file123"))
        }
    }

    #[cfg(test)]
    mod case_tests {
        use super::*;
        #[test]
        fn test_keep_case() {
            let file = String::from("test file");
            let opt = CaseOptions {
                case: Case::Keep,
                snake: false,
            };
            let result = opt.process(&file);
            assert_eq!(result, String::from("test file"));
        }

        #[test]
        fn test_keep_case_snake() {
            let file = String::from("test file");
            let opt = CaseOptions {
                case: Case::Keep,
                snake: true,
            };
            let result = opt.process(&file);
            assert_eq!(result, String::from("test_file"));
        }

        #[test]
        fn test_lower_case() {
            let file = String::from("TEST FILE");
            let opt = CaseOptions {
                case: Case::Lower,
                snake: false,
            };
            let result = opt.process(&file);
            assert_eq!(result, String::from("test file"));
        }

        #[test]
        fn test_lower_case_snake() {
            let file = String::from("TEST FILE");
            let opt = CaseOptions {
                case: Case::Lower,
                snake: true,
            };
            let result = opt.process(&file);
            assert_eq!(result, String::from("test_file"));
        }

        #[test]
        fn test_upper_case() {
            let file = String::from("test file");
            let opt = CaseOptions {
                case: Case::Upper,
                snake: false,
            };
            let result = opt.process(&file);
            assert_eq!(result, String::from("TEST FILE"));
        }

        #[test]
        fn test_upper_case_snake() {
            let file = String::from("test file");
            let opt = CaseOptions {
                case: Case::Upper,
                snake: true,
            };
            let result = opt.process(&file);
            assert_eq!(result, String::from("TEST_FILE"));
        }

        #[test]
        fn test_title_case() {
            let file = String::from("test file");
            let opt = CaseOptions {
                case: Case::Title,
                snake: false,
            };
            let result = opt.process(&file);
            assert_eq!(result, String::from("Test File"));
        }

        #[test]
        fn test_title_case_snake() {
            let file = String::from("test file");
            let opt = CaseOptions {
                case: Case::Title,
                snake: true,
            };
            let result = opt.process(&file);
            assert_eq!(result, String::from("Test_File"));
        }

        #[test]
        fn test_sentence_case() {
            let file = String::from("test file");
            let opt = CaseOptions {
                case: Case::Sentence,
                snake: false,
            };
            let result = opt.process(&file);
            assert_eq!(result, String::from("Test file"));
        }

        #[test]
        fn test_sentence_case_snake() {
            let file = String::from("test file");
            let opt = CaseOptions {
                case: Case::Sentence,
                snake: true,
            };
            let result = opt.process(&file);
            assert_eq!(result, String::from("Test_file"));
        }
    }
}
