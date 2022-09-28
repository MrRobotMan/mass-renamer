use crate::file::{Process, RenameFile};

/// Options for the name feature.
pub enum NameOptions {
    Keep,
    Remove,
    Fixed(String),
    Reverse,
}

impl Process for NameOptions {
    /// Using the `NameOptions` enum and the name function, return a modified string.
    /// - `Keep` - Do not change the original file name (default).
    /// - `Remove` - Completely erase the file from the selected items. This allows it to be rebuilt using components higher than (2).
    /// - `Fixed` - Specify a new file in the box for all selected items. Only really useful if you're also using the Numbering section.
    /// - `Reverse` - Reverse the name, e.g. 12345.txt becomes 54321.txt.
    fn process(&self, file: &mut RenameFile) {
        match self {
            NameOptions::Keep => (),
            NameOptions::Remove => file.stem = "".to_owned(),
            NameOptions::Fixed(x) => file.stem = x.to_owned(),
            NameOptions::Reverse => file.stem = file.stem.chars().rev().collect::<String>(),
        };
    }
}

#[cfg(test)]
mod name_tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn keep_name() {
        let file = RenameFile::new(Path::new("file")).unwrap();
        let opt = NameOptions::Keep;
        opt.process(&mut file);
        assert_eq!(file.stem, String::from("file"));
    }
    #[test]
    fn remove_name() {
        let file = RenameFile::new(Path::new("file")).unwrap();
        let opt = NameOptions::Remove;
        opt.process(&mut file);
        assert_eq!(file.stem, String::from(""));
    }
    #[test]
    fn fixed_name() {
        let file = RenameFile::new(Path::new("file")).unwrap();
        let new_name = "renamed_file";
        let opt = NameOptions::Fixed(new_name.to_owned());
        opt.process(&mut file);
        assert_eq!(file.stem, String::from(new_name));
    }
    #[test]
    fn reverse_name() {
        let file = RenameFile::new(Path::new("file")).unwrap();
        let reversed = String::from("elif");
        let opt = NameOptions::Reverse;
        opt.process(&mut file);
        assert_eq!(file.stem, reversed);
    }
}
