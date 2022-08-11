#![allow(dead_code)] // Must disable later when project is complete.

pub mod renamer {
    use super::{name, regex};
    use eyre::{eyre, Result};
    use std::path::{Path, PathBuf};

    pub fn rename_file(
        file: &Path,
        modes: (Option<regex::Options>, Option<name::Options>),
    ) -> Result<PathBuf> {
        let mut parent = if let Some(name) = file.parent() {
            name.to_owned()
        } else {
            PathBuf::from("")
        };
        let mut new_name = {
            if let Some(opt) = modes.0 {
                regex::match_and_replace(opt.exp, opt.rep, &file, opt.extension)?
            } else {
                if let Some(s) = file.to_str() {
                    String::from(s)
                } else {
                    return Err(eyre!("Failed to extract file."));
                }
            }
        };
        if let Some(opt) = modes.1 {
            new_name = name::name(&new_name, opt);
        };
        parent.push(new_name);
        Ok(parent)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::path::{Path, PathBuf};
        #[test]
        fn test_regex() {
            let file = Path::new("Testfile123.txt");
            let expected = PathBuf::from("TestfileABC.txt");
            let opt = regex::Options {
                exp: "123",
                rep: "ABC",
                extension: true,
            };
            let modes = (Some(opt), None);
            let result = rename_file(file, modes);
            assert_eq!(result.unwrap(), expected)
        }
    }
}

pub mod name {
    pub enum Options<'a> {
        Keep,
        Remove,
        Fixed(&'a str),
        Reverse,
    }

    /// Using the `Options` enum and the name function, return a modified string.
    /// - `Keep` - Do not change the original file name (default).
    /// - `Remove` - Completely erase the file from the selected items. This allows it to be rebuilt using components higher than (2).
    /// - `Fixed` - Specify a new file in the box for all selected items. Only really useful if you're also using the Numbering section.
    /// - `Reverse` - Reverse the name, e.g. 12345.txt becomes 54321.txt.
    pub fn name(file: &str, mode: Options) -> String {
        match mode {
            Options::Keep => file.to_owned(),
            Options::Remove => "".to_owned(),
            Options::Fixed(x) => String::from(x),
            Options::Reverse => file.chars().rev().collect::<String>(),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn keep_name() {
            let file = String::from("file");
            let result = name(&file, Options::Keep);
            assert_eq!(result, String::from("file"));
        }
        #[test]
        fn remove_name() {
            let file = String::from("file");
            let result = name(&file, Options::Remove);
            assert_eq!(result, String::from(""));
        }
        #[test]
        fn fixed_name() {
            let file = String::from("file");
            let new_name = "renamed_file";
            let result = name(&file, Options::Fixed(new_name));
            assert_eq!(result, String::from(new_name));
        }
        #[test]
        fn reverse_name() {
            let file = String::from("file");
            let reversed = String::from("elif");
            let result = name(&file, Options::Reverse);
            assert_eq!(result, reversed);
        }
    }
}

pub mod regex {
    use eyre::{eyre, Result};
    use regex::Regex;
    use std::path::Path;
    pub struct Options<'a> {
        pub exp: &'a str,
        pub rep: &'a str,
        pub extension: bool,
    }

    /// Use a regular expression `Match` to find the offending text and `Replace` it with new.
    /// Check the `Include Ext.` box to include the file extension in the `Match`.
    pub fn match_and_replace(exp: &str, rep: &str, file: &Path, extension: bool) -> Result<String> {
        let exp = Regex::new(exp)?;
        let base = if extension {
            file.file_name()
        } else {
            file.file_stem()
        };
        let ext = file.extension();
        if let Some(s) = base {
            if let Some(s) = s.to_str() {
                let mut res = exp.replace_all(s, rep).to_string();
                if !extension {
                    if let Some(e) = ext {
                        res.push_str(".");
                        if let Some(e) = e.to_str() {
                            res.push_str(e);
                        } else {
                            return Err(eyre!("Extension could not be converted to str."));
                        };
                    } else {
                        return Err(eyre!("Extension does not exist."));
                    }
                }
                Ok(res)
            } else {
                Err(eyre!(
                    "Could not convert file to str in regex match, likely invalid unicode."
                ))
            }
        } else {
            Err(eyre!("Bad file in regex match."))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use eyre;
        use std::path::Path;
        #[test]
        fn regex_test_with_extension() {
            let exp = "0123.txt";
            let rep = "ABCD.csv";
            let file = Path::new("file0123.txt");
            let result = match_and_replace(exp, rep, file, true).unwrap();
            assert_eq!(result, String::from("fileABCD.csv"));
        }
        #[test]
        fn regex_test_no_extension() {
            let exp = "0123";
            let rep = "ABCD";
            let file = Path::new("file0123.txt");
            let result = match_and_replace(exp, rep, file, false).unwrap();
            assert_eq!(result, String::from("fileABCD.txt"));
        }
        #[test]
        fn regex_test_no_extension_no_match() {
            let exp = "0123";
            let rep = "ABCD";
            let file = Path::new("file123.txt");
            let result = match_and_replace(exp, rep, file, false).unwrap();
            assert_eq!(result, String::from("file123.txt"));
        }
        #[test]
        fn bad_file_with_extension() {
            let exp = "";
            let rep = "";
            let file = Path::new("");
            let result = match_and_replace(exp, rep, file, true).unwrap_err();
            let expected = eyre::Report::msg("Bad file in regex match.").to_string();
            assert_eq!(result.to_string(), expected)
        }
        #[test]
        fn bad_file_no_extension() {
            let exp = "";
            let rep = "";
            let file = Path::new("");
            let result = match_and_replace(exp, rep, file, false).unwrap_err();
            let expected = eyre::Report::msg("Bad file in regex match.").to_string();
            assert_eq!(result.to_string(), expected)
        }
    }
}

pub mod rename {
    /// Options for basic renaming rules.
    /// replace: text to be replaced
    /// with: new text
    /// file: name of file to rename
    /// case: true for case sensitive, false for case-insensitive
    pub struct Options<'a> {
        pub replace: &'a str,
        pub with: &'a str,
        pub file: &'a str,
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
    mod tests {
        use super::*;

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
