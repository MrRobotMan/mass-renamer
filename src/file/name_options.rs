/// Options for the name feature.
pub enum NameOptions<'a> {
    Keep,
    Remove,
    Fixed(&'a str),
    Reverse,
}

impl NameOptions<'_> {
    /// Using the `NameOptions` enum and the name function, return a modified string.
    /// - `Keep` - Do not change the original file name (default).
    /// - `Remove` - Completely erase the file from the selected items. This allows it to be rebuilt using components higher than (2).
    /// - `Fixed` - Specify a new file in the box for all selected items. Only really useful if you're also using the Numbering section.
    /// - `Reverse` - Reverse the name, e.g. 12345.txt becomes 54321.txt.
    pub fn process(&self, file: &str) -> String {
        match self {
            NameOptions::Keep => file.to_owned(),
            NameOptions::Remove => "".to_owned(),
            NameOptions::Fixed(x) => String::from(*x),
            NameOptions::Reverse => file.chars().rev().collect::<String>(),
        }
    }
}

#[cfg(test)]
mod name_tests {
    use super::*;
    #[test]
    fn keep_name() {
        let file = String::from("file");
        let opt = NameOptions::Keep;
        let result = opt.process(&file);
        assert_eq!(result, String::from("file"));
    }
    #[test]
    fn remove_name() {
        let file = String::from("file");
        let opt = NameOptions::Remove;
        let result = opt.process(&file);
        assert_eq!(result, String::from(""));
    }
    #[test]
    fn fixed_name() {
        let file = String::from("file");
        let new_name = "renamed_file";
        let opt = NameOptions::Fixed(new_name);
        let result = opt.process(&file);
        assert_eq!(result, String::from(new_name));
    }
    #[test]
    fn reverse_name() {
        let file = String::from("file");
        let reversed = String::from("elif");
        let opt = NameOptions::Reverse;
        let result = opt.process(&file);
        assert_eq!(result, reversed);
    }
}
