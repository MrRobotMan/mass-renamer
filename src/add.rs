use crate::{Process, RenameFile};

/// Add a fixed `Prefix` or`Suffix` to the filename,
/// or `Insert` text at a specific location (0 indexed, negative to index from the end).
///
/// You may also choose to add a `Word Space`. This will insert a space before any
/// capital letter (except the first character), unless there's a space already there.
pub struct AddOptions<'a> {
    pub prefix: Option<&'a str>,
    pub insert: Option<(i32, &'a str)>,
    pub suffix: Option<&'a str>,
    pub word_space: bool,
}

impl Process for AddOptions<'_> {
    fn process(&self, file: &mut RenameFile) {
        let file = &mut file.stem;
        if let Some(prefix) = self.prefix {
            file.insert_str(0, prefix);
        }

        if let Some((pos, insert)) = self.insert {
            match pos {
                p if p >= file.len() as i32 => file.push_str(insert),
                p if p >= 0 => file.insert_str(p as usize, insert),
                p if -p >= file.len() as i32 => file.insert_str(0, insert),
                _ => {
                    let p = (file.len() as i32 + pos) as usize;
                    file.insert_str(p, insert);
                } // pos is negative
            }
        }

        if let Some(suffix) = self.suffix {
            file.push_str(suffix);
        }

        if self.word_space {
            let mut new = String::new();
            for chr in file.chars() {
                if chr.is_uppercase() {
                    new.push(' ');
                }
                new.push(chr);
            }
            *file = new
        }
    }
}

#[cfg(test)]
mod add_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn add_all_options() {
        let prefix = Some("prefix-");
        let insert = Some((15, "-insert-"));
        let suffix = Some("-suffix");
        let word_space = true;
        let file = Path::new("SomeTestFile");
        let opt = AddOptions {
            prefix,
            insert,
            suffix,
            word_space,
        };
        let mut rename = RenameFile::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(
            rename.stem,
            "prefix- Some Test-insert- File-suffix".to_owned()
        )
    }

    #[test]
    fn test_negative_insert() {
        let insert = Some((-1, "!"));
        let file = Path::new("Some Test File");
        let opt = AddOptions {
            prefix: None,
            insert,
            suffix: None,
            word_space: false,
        };
        let mut rename = RenameFile::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(rename.stem, "Some Test Fil!e".to_owned());
    }

    #[test]
    fn test_insert_too_far_positive() {
        let insert = Some((100, "!"));
        let file = Path::new("Some Test File");
        let opt = AddOptions {
            prefix: None,
            insert,
            suffix: None,
            word_space: false,
        };
        let mut rename = RenameFile::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(rename.stem, "Some Test File!".to_owned());
    }

    #[test]
    fn test_insert_too_far_negative() {
        let insert = Some((-100, "!"));
        let file = Path::new("Some Test File");
        let opt = AddOptions {
            prefix: None,
            insert,
            suffix: None,
            word_space: false,
        };
        let mut rename = RenameFile::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(rename.stem, "!Some Test File".to_owned());
    }
}
