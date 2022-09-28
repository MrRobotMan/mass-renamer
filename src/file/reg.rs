use crate::file::{Process, RenameFile};
use regex::Regex;

/// Options for the regex feature
pub struct RegexOptions<'a> {
    pub exp: &'a str,
    pub rep: &'a str,
    pub extension: bool,
}

impl Process for RegexOptions<'_> {
    /// Use a regular expression `Match` to find the offending text and `Replace` it with new.
    fn process(&self, file: &mut RenameFile) -> () {
        if let Ok(exp) = Regex::new(self.exp) {
            match (self.extension, file.extension) {
                (false, _) | (_, None) => {
                    file.stem = exp.replace_all(&file.stem, self.rep).to_string()
                }
                (true, Some(ext)) => {
                    let mut f = file.stem.clone();
                    f.push('.');
                    f.push_str(&ext);
                    let res = exp.replace_all(&f, self.rep).to_string();
                    match res.rsplit_once('.') {
                        None => file.stem = res,
                        Some((s, e)) => {
                            file.stem = s.to_owned();
                            file.extension = Some(e.to_owned());
                        }
                    }
                }
            }
        };
    }
}

#[cfg(test)]
mod regex_tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn regex_test_with_extension() {
        let exp = "0123.txt";
        let file = Path::new("./file0123.txt");
        let rep = "ABCD.csv";
        let opt = RegexOptions {
            exp,
            rep,
            extension: true,
        };
        let rename = RenameFile::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(
            (rename.stem, rename.extension),
            (String::from("fileABCD"), Some(String::from(".csv")))
        );
    }
    #[test]
    fn regex_test_no_extension() {
        let exp = "0123";
        let rep = "ABCD";
        let file = Path::new("./file0123.txt");
        let opt = RegexOptions {
            exp,
            rep,
            extension: false,
        };
        let rename = RenameFile::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(
            (rename.stem, rename.extension),
            (String::from("fileABCD"), Some(String::from(".txt")))
        );
    }
    #[test]
    fn regex_test_no_extension_no_match() {
        let exp = "0123";
        let rep = "ABCD";
        let file = Path::new("./file123.txt");
        let opt = RegexOptions {
            exp,
            rep,
            extension: false,
        };
        let rename = RenameFile::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(
            (rename.stem, rename.extension),
            (String::from("file123"), Some(String::from(".txt")))
        );
    }
}
