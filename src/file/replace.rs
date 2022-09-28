use crate::file::{Process, RenameFile};

/// Options for basic renaming rules.
/// replace: text to be replaced
/// with: new text
/// case: true for case sensitive, false for case-insensitive
pub struct ReplaceOptions<'a> {
    pub replace: &'a str,
    pub with: &'a str,
    pub case: bool,
}

impl Process for ReplaceOptions<'_> {
    /// `Replace` the text in this field with the text in the `With` field.
    /// `Replace` can be case-sensitive using `Match Case` checkbox.
    /// Note that the `With` text is always replaced with the text as written, including any specific text case.
    fn process(&self, file: &mut RenameFile) {
        let file = &mut file.stem;
        if self.case {
            *file = file.replace(self.replace, self.with);
        } else {
            let start = file.to_lowercase().find(&self.replace.to_lowercase());
            let span = self.replace.len();
            match start {
                Some(idx) => {
                    for _ in idx..(idx + span) {
                        file.remove(idx);
                    }
                    file.insert_str(idx, self.with);
                }
                None => (),
            }
        }
    }
}

#[cfg(test)]
mod match_tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn no_matching_text_case_sensitive() {
        let replace = "ABC";
        let with = "123";
        let mut file = RenameFile::new(Path::new("fileabc")).unwrap();
        let case = true;
        let opt = ReplaceOptions {
            replace,
            with,
            case,
        };
        opt.process(&mut file);
        assert_eq!(file.stem, String::from("fileabc"))
    }
    #[test]
    fn no_matching_text_case_insensitive() {
        let replace = "qrs";
        let with = "123";
        let mut file = RenameFile::new(Path::new("fileabc")).unwrap();
        let case = false;
        let opt = ReplaceOptions {
            replace,
            with,
            case,
        };
        opt.process(&mut file);
        assert_eq!(file.stem, String::from("fileabc"))
    }
    #[test]
    fn matched_case_sensitive() {
        let replace = "abc";
        let with = "123";
        let mut file = RenameFile::new(Path::new("fileabc")).unwrap();
        let case = true;
        let opt = ReplaceOptions {
            replace,
            with,
            case,
        };
        opt.process(&mut file);
        assert_eq!(file.stem, String::from("file123"))
    }
    #[test]
    fn matched_case_insensitive() {
        let replace = "ABC";
        let with = "123";
        let mut file = RenameFile::new(Path::new("fileabc")).unwrap();
        let case = false;
        let opt = ReplaceOptions {
            replace,
            with,
            case,
        };
        opt.process(&mut file);
        assert_eq!(file.stem, String::from("file123"))
    }
}
