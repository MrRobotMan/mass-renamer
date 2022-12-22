use crate::{Process, RenameFile};

/// Options for basic renaming rules.
/// - `replace` - text to be replaced
/// - `with` - new text. Note: the text is always replaced with the text as written, including any specific text case.
/// - `case` - true for case sensitive, false for case-insensitive
pub struct ReplaceOptions<'a> {
    pub replace: &'a str,
    pub with: &'a str,
    pub case: bool,
}

impl Process for ReplaceOptions<'_> {
    fn process(&self, file: &mut RenameFile) {
        let file = &mut file.stem;
        if self.case {
            *file = file.replace(self.replace, self.with);
        } else {
            let start = file.to_lowercase().find(&self.replace.to_lowercase());
            let span = self.replace.len();
            if let Some(idx) = start {
                for _ in idx..(idx + span) {
                    file.remove(idx);
                }
                file.insert_str(idx, self.with);
            };
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
