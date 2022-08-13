#![allow(dead_code)] // Must disable later when project is complete.

pub mod renamer {
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
            Option<String>,
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
                match_and_replace(opt.exp, opt.rep, &file, opt.extension)?
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
            new_name = name(&new_name, opt);
        };
        if let Some(opt) = modes.2 {
            new_name = replace(opt.replace, opt.with, &new_name, opt.case)
        }
        parent.push(new_name);
        if let Some(e) = ext {
            Ok(parent.with_extension(e))
        } else {
            Ok(parent)
        }
    }

    /// Options for uses the name feature.
    pub enum NameOptions<'a> {
        Keep,
        Remove,
        Fixed(&'a str),
        Reverse,
    }

    /// Using the `NameOptions` enum and the name function, return a modified string.
    /// - `Keep` - Do not change the original file name (default).
    /// - `Remove` - Completely erase the file from the selected items. This allows it to be rebuilt using components higher than (2).
    /// - `Fixed` - Specify a new file in the box for all selected items. Only really useful if you're also using the Numbering section.
    /// - `Reverse` - Reverse the name, e.g. 12345.txt becomes 54321.txt.
    fn name(file: &str, mode: NameOptions) -> String {
        match mode {
            NameOptions::Keep => file.to_owned(),
            NameOptions::Remove => "".to_owned(),
            NameOptions::Fixed(x) => String::from(x),
            NameOptions::Reverse => file.chars().rev().collect::<String>(),
        }
    }

    /// Options for the regex feature
    pub struct RegexOptions<'a> {
        pub exp: &'a str,
        pub rep: &'a str,
        pub extension: bool,
    }

    /// Use a regular expression `Match` to find the offending text and `Replace` it with new.
    /// Check the `Include Ext.` box to include the file extension in the `Match`.
    fn match_and_replace(
        exp: &str,
        rep: &str,
        file: &Path,
        extension: bool,
    ) -> Result<(String, Option<String>), Box<dyn Error>> {
        let exp = Regex::new(exp)?;
        let base = if extension {
            file.file_name()
        } else {
            file.file_stem()
        };
        let ext = file.extension();
        if let Some(s) = base {
            if let Some(s) = s.to_str() {
                let res = exp.replace_all(s, rep).to_string();
                if !extension {
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
                Err("Could not convert file to str in regex match, likely invalid unicode.".into())
            }
        } else {
            Err("Bad file in regex match.".into())
        }
    }

    /// Options for basic renaming rules.
    /// replace: text to be replaced
    /// with: new text
    /// file: name of file to rename
    /// case: true for case sensitive, false for case-insensitive
    pub struct ReplaceOptions<'a> {
        pub replace: &'a str,
        pub with: &'a str,
        pub case: bool,
    }

    /// `Replace` the text in this field with the text in the `With` field.
    /// `Replace` can be case-sensitive using `Match Case` checkbox.
    /// Note that the `With` text is always replaced with the text as written, including any specific text case.
    pub fn replace(search: &str, with: &str, file: &str, case: bool) -> String {
        let mut result = String::from(file);
        if case {
            result.replace(search, with)
        } else {
            let start = file.to_lowercase().find(&search.to_lowercase());
            let span = search.len();
            match start {
                Some(idx) => {
                    for _ in idx..(idx + span) {
                        result.remove(idx);
                    }
                    result.insert_str(idx, with);
                    result
                }
                None => result,
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
    mod tests {
        use super::*;
        use std::path::Path;
        #[test]
        fn regex_test_with_extension() {
            let exp = "0123.txt";
            let rep = "ABCD.csv";
            let file = Path::new("file0123.txt");
            let result = match_and_replace(exp, rep, file, true).unwrap();
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
            let result = match_and_replace(exp, rep, file, false).unwrap();
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
            let result = match_and_replace(exp, rep, file, false).unwrap();
            assert_eq!(result, (String::from("file123"), Some(String::from("txt"))));
        }
        #[test]
        fn bad_file_with_extension() {
            let exp = "";
            let rep = "";
            let file = Path::new("");
            let result = match_and_replace(exp, rep, file, true).unwrap_err();
            assert_eq!(result.to_string(), "Bad file in regex match.")
        }
        #[test]
        fn bad_file_no_extension() {
            let exp = "";
            let rep = "";
            let file = Path::new("");
            let result = match_and_replace(exp, rep, file, false).unwrap_err();
            assert_eq!(result.to_string(), "Bad file in regex match.")
        }
        #[test]
        fn keep_name() {
            let file = String::from("file");
            let result = name(&file, NameOptions::Keep);
            assert_eq!(result, String::from("file"));
        }
        #[test]
        fn remove_name() {
            let file = String::from("file");
            let result = name(&file, NameOptions::Remove);
            assert_eq!(result, String::from(""));
        }
        #[test]
        fn fixed_name() {
            let file = String::from("file");
            let new_name = "renamed_file";
            let result = name(&file, NameOptions::Fixed(new_name));
            assert_eq!(result, String::from(new_name));
        }
        #[test]
        fn reverse_name() {
            let file = String::from("file");
            let reversed = String::from("elif");
            let result = name(&file, NameOptions::Reverse);
            assert_eq!(result, reversed);
        }

        #[test]
        fn no_matching_text_case_sensitive() {
            let search = "ABC";
            let with = "123";
            let file = "fileabc";
            let case = true;
            let result = replace(search, with, file, case);
            assert_eq!(result, String::from(file))
        }
        #[test]
        fn no_matching_text_case_insensitive() {
            let search = "qrs";
            let with = "123";
            let file = "fileabc";
            let case = false;
            let result = replace(search, with, file, case);
            assert_eq!(result, String::from(file))
        }
        #[test]
        fn matched_case_sensitive() {
            let search = "abc";
            let with = "123";
            let file = "fileabc";
            let case = true;
            let result = replace(search, with, file, case);
            assert_eq!(result, String::from("file123"))
        }
        #[test]
        fn matched_case_insensitive() {
            let search = "ABC";
            let with = "123";
            let file = "fileabc";
            let case = false;
            let result = replace(search, with, file, case);
            assert_eq!(result, String::from("file123"))
        }
    }
}
