use crate::file::Process;

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
    fn process(&self, file: &mut String) {
        match self {
            NameOptions::Keep => (),
            NameOptions::Remove => *file = "".to_owned(),
            NameOptions::Fixed(x) => *file = x.to_owned(),
            NameOptions::Reverse => *file = file.chars().rev().collect::<String>(),
        };
    }
}

#[cfg(test)]
mod name_tests {
    use super::*;
    #[test]
    fn keep_name() {
        let mut file = String::from("file");
        let opt = NameOptions::Keep;
        opt.process(&mut file);
        assert_eq!(file, String::from("file"));
    }
    #[test]
    fn remove_name() {
        let mut file = String::from("file");
        let opt = NameOptions::Remove;
        opt.process(&mut file);
        assert_eq!(file, String::from(""));
    }
    #[test]
    fn fixed_name() {
        let mut file = String::from("file");
        let new_name = "renamed_file";
        let opt = NameOptions::Fixed(new_name.to_owned());
        opt.process(&mut file);
        assert_eq!(file, String::from(new_name));
    }
    #[test]
    fn reverse_name() {
        let mut file = String::from("file");
        let reversed = String::from("elif");
        let opt = NameOptions::Reverse;
        opt.process(&mut file);
        assert_eq!(file, reversed);
    }
}
