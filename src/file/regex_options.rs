use regex::Regex;

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
        file: &mut String,
        ext: &mut String,
        // extension: bool,
    ) -> () {
        if let Ok(exp) = Regex::new(self.exp) {
            match self.extension {
                false => *file = exp.replace_all(&file, self.rep).to_string(),
                true => {
                    let mut f = file.clone();
                    f.push_str(ext);
                    let res = exp.replace_all(&f, self.rep).to_string();
                    let split = res.rsplit_once(".");
                    match split {
                        None => *file = res,
                        Some(r) => {
                            *file = String::from(r.0);
                            *ext = String::from(format!("{}{}", ".", r.1));
                        }
                    }
                }
            }
        };
    }
    // if let Some(s) = base {
    //     if let Some(s) = s.to_str() {
    //         let res = exp.replace_all(s, self.rep).to_string();
    //         if !self.extension {
    //             if let Some(e) = ext {
    //                 if let Some(e) = e.to_str() {
    //                     return Ok((res, Some(String::from(e))));
    //                 } else {
    //                     return Err("Extension could not be converted to str.".into());
    //                 };
    //             } else {
    //                 return Err("Extension does not exist.".into());
    //             }
    //         }
    //         if let Some(new_name) = res.rsplit_once(".") {
    //             return Ok((new_name.0.to_string(), Some(new_name.1.to_string())));
    //         } else {
    //             Ok((res, None))
    //         }
    //     } else {
    //         Err("Could not convert file to str in regex match, likely invalid unicode.".into())
    //     }
    // } else {
    //     Err("Bad file in regex match.".into())
    // }
}

#[cfg(test)]
mod regex_tests {
    use super::*;
    #[test]
    fn regex_test_with_extension() {
        let exp = "0123.txt";
        let mut file = String::from("file0123");
        let mut ext = String::from(".txt");
        let rep = "ABCD.csv";
        let opt = RegexOptions {
            exp,
            rep,
            extension: true,
        };
        opt.process(&mut file, &mut ext);
        assert_eq!(
            (file, ext),
            (String::from("fileABCD"), String::from(".csv"))
        );
    }
    #[test]
    fn regex_test_no_extension() {
        let exp = "0123";
        let rep = "ABCD";
        let mut file = String::from("file0123");
        let mut ext = String::from(".txt");
        let opt = RegexOptions {
            exp,
            rep,
            extension: false,
        };
        opt.process(&mut file, &mut ext);
        assert_eq!(
            (file, ext),
            (String::from("fileABCD"), String::from(".txt"))
        );
    }
    #[test]
    fn regex_test_no_extension_no_match() {
        let exp = "0123";
        let rep = "ABCD";
        let mut file = String::from("file123");
        let mut ext = String::from(".txt");
        let opt = RegexOptions {
            exp,
            rep,
            extension: false,
        };
        opt.process(&mut file, &mut ext);
        assert_eq!((file, ext), (String::from("file123"), String::from(".txt")));
    }
    #[test]
    fn bad_file_with_extension() {
        let exp = "";
        let rep = "";
        let mut file = String::from("");
        let mut ext = String::from("");
        let opt = RegexOptions {
            exp,
            rep,
            extension: true,
        };
        opt.process(&mut file, &mut ext);
        assert_eq!((file, ext), (String::from(""), String::from("")))
    }
    #[test]
    fn bad_file_no_extension() {
        let exp = "";
        let rep = "";
        let mut file = String::from("");
        let mut ext = String::from("");
        let opt = RegexOptions {
            exp,
            rep,
            extension: false,
        };
        opt.process(&mut file, &mut ext);
        assert_eq!((file, ext), (String::from(""), String::from("")))
    }
}
