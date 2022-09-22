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
    pub fn process(&self, file: &str) -> String {
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
