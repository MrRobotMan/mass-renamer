use regex::Regex;
use std::{error::Error, path::Path};

/// Options for the regex feature
pub struct RegexOptions<'a> {
    pub exp: &'a str,
    pub rep: &'a str,
    pub extension: bool,
}

impl RegexOptions<'_> {
    /// Use a regular expression `Match` to find the offending text and `Replace` it with new.
    /// Check the `Include Ext.` box to include the file extension in the `Match`.
    pub fn process(
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
                Err("Could not convert file to str in regex match, likely invalid unicode.".into())
            }
        } else {
            Err("Bad file in regex match.".into())
        }
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
