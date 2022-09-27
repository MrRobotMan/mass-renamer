use crate::file::Process;
use regex::Regex;

/// Options for the regex feature
pub struct RegexOptions<'a> {
    pub exp: &'a str,
    pub rep: &'a str,
}

impl Process for RegexOptions<'_> {
    /// Use a regular expression `Match` to find the offending text and `Replace` it with new.
    fn process(&self, file: &mut String) -> () {
        if let Ok(exp) = Regex::new(self.exp) {
            *file = exp.replace_all(&file, self.rep).to_string();
        };
    }
}

#[cfg(test)]
mod regex_tests {
    use super::*;

    #[test]
    fn regex_test_success() {
        let exp = "0123";
        let rep = "ABCD";
        let mut file = String::from("file0123");
        let opt = RegexOptions { exp, rep };
        opt.process(&mut file);
        assert_eq!(file, String::from("fileABCD"));
    }
    #[test]
    fn regex_test_no_match() {
        let exp = "0123";
        let rep = "ABCD";
        let mut file = String::from("file123");
        let opt = RegexOptions { exp, rep };
        opt.process(&mut file);
        assert_eq!(file, String::from("file123"));
    }

    #[test]
    fn bad_file_no_extension() {
        let exp = "";
        let rep = "";
        let mut file = String::from("");
        let opt = RegexOptions { exp, rep };
        opt.process(&mut file);
        assert_eq!(file, String::from(""))
    }
}
