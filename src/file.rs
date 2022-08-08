pub mod renamer {
    use super::{name, regex};
    use eyre::{eyre, Result};
    use std::path::Path;

    pub fn rename_file(
        filename: &Path,
        modes: (Option<regex::Options>, Option<name::NameMode>),
    ) -> Result<String> {
        let mut new_name = {
            if let Some(opt) = modes.0 {
                regex::match_and_replace(opt.exp, opt.rep, &filename, opt.extension)?
            } else {
                if let Some(s) = filename.to_str() {
                    String::from(s)
                } else {
                    return Err(eyre!("Failed to extract filename."));
                }
            }
        };
        if let Some(opt) = modes.1 {
            new_name = name::name(&new_name, opt);
        };
        Ok(new_name)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::path::Path;
        #[test]
        fn test_regex() {
            let filename = Path::new("Testfile123.txt");
            let expected = String::from("TestfileABC.txt");
            let opt = regex::Options {
                exp: "123",
                rep: "ABC",
                extension: true,
            };
            let modes = (Some(opt), None);
            let result = rename_file(filename, modes);
            assert_eq!(result.unwrap(), expected)
        }
    }
}

pub mod name {
    pub enum NameMode<'a> {
        Keep,
        Remove,
        Fixed(&'a str),
        Reverse,
    }

    /// Using the `NameMode` enum and the name function, return a modified string.
    /// - `Keep` - Do not change the original file name (default).
    /// - `Remove` - Completely erase the filename from the selected items. This allows it to be rebuilt using components higher than (2).
    /// - `Fixed` - Specify a new filename in the box for all selected items. Only really useful if you're also using the Numbering section.
    /// - `Reverse` - Reverse the name, e.g. 12345.txt becomes 54321.txt.
    pub fn name(filename: &str, mode: NameMode) -> String {
        match mode {
            NameMode::Keep => filename.to_owned(),
            NameMode::Remove => "".to_owned(),
            NameMode::Fixed(x) => String::from(x),
            NameMode::Reverse => filename.chars().rev().collect::<String>(),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn keep_name() {
            let filename = String::from("file");
            let result = name(&filename, NameMode::Keep);
            assert_eq!(result, String::from("file"));
        }
        #[test]
        fn remove_name() {
            let filename = String::from("file");
            let result = name(&filename, NameMode::Remove);
            assert_eq!(result, String::from(""));
        }
        #[test]
        fn fixed_name() {
            let filename = String::from("file");
            let new_name = "renamed_file";
            let result = name(&filename, NameMode::Fixed(new_name));
            assert_eq!(result, String::from(new_name));
        }
        #[test]
        fn reverse_name() {
            let filename = String::from("file");
            let reversed = String::from("elif");
            let result = name(&filename, NameMode::Reverse);
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
    pub fn match_and_replace(
        exp: &str,
        rep: &str,
        filename: &Path,
        extension: bool,
    ) -> Result<String> {
        let exp = Regex::new(exp)?;
        let base = if extension {
            filename.file_name()
        } else {
            filename.file_stem()
        };
        let ext = filename.extension();
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
                    "Could not convert filename to str in regex match, likely invalid unicode."
                ))
            }
        } else {
            Err(eyre!("Bad filename in regex match."))
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
            let filename = Path::new("file0123.txt");
            let result = match_and_replace(exp, rep, filename, true).unwrap();
            assert_eq!(result, String::from("fileABCD.csv"));
        }
        #[test]
        fn regex_test_no_extension() {
            let exp = "0123";
            let rep = "ABCD";
            let filename = Path::new("file0123.txt");
            let result = match_and_replace(exp, rep, filename, false).unwrap();
            assert_eq!(result, String::from("fileABCD.txt"));
        }
        #[test]
        fn regex_test_no_extension_no_match() {
            let exp = "0123";
            let rep = "ABCD";
            let filename = Path::new("file123.txt");
            let result = match_and_replace(exp, rep, filename, false).unwrap();
            assert_eq!(result, String::from("file123.txt"));
        }
        #[test]
        fn bad_file_with_extension() {
            let exp = "";
            let rep = "";
            let filename = Path::new("");
            let result = match_and_replace(exp, rep, filename, true).unwrap_err();
            let expected = eyre::Report::msg("Bad filename in regex match.").to_string();
            assert_eq!(result.to_string(), expected)
        }
        #[test]
        fn bad_file_no_extension() {
            let exp = "";
            let rep = "";
            let filename = Path::new("");
            let result = match_and_replace(exp, rep, filename, false).unwrap_err();
            let expected = eyre::Report::msg("Bad filename in regex match.").to_string();
            assert_eq!(result.to_string(), expected)
        }
    }
}
