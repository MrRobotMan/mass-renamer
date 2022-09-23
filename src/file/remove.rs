/// Options for removing parts of the filename.
/// Remove specific parts of a filename but not file extensions.
///
/// - `First n` - Remove the first n characters from the name.
/// - `Last n` - Remove the last n characters from the name.
/// - `From`/`to` - Remove a string of text, e.g. from the 6th to the 9th characters (0 indexed).
/// - `Chars` - Remove occurrences of the listed characters from the name (no separator needed).
/// - `Words` - Remove occurrences of listed words (separated by spaces).
/// - `Crop` - Remove any text which occurs before (or after) a specific character or word.
/// - `Digits` - Remove all occurrences of the digits 0-9 from the filename.
/// - `High` - Remove high-ASCII characters (chars from 128 to 255).
/// - `Trim` - Remove leading and trailing spaces.
/// - `D/S` - Remove occurrences of double spaces, and replace them with single spaces.
/// - `Accent` - Remove accented characters and replace them with non-accented versions.
/// - `Chars` - Remove all characters.
/// - `Sym` - Remove all symbols.
/// - `Lead Dots` - Remove the . or .. from the front of filenames.
///
/// Note: When you use the `words` option, you have the ability of specifying a special
/// value using the wildcard (*). This will remove the specified string, and any
/// characters occupied by the wildcard. So for example, specifying [*] would convert
/// "Hello[ABC] Joe" to just "Hello Joe", as it has removed the two square brackets and
/// everything between.

pub struct RemoveOptions<'a> {
    first_n: usize,
    last_n: usize,
    from: usize,
    to: usize,
    chars: Option<&'a str>,
}

use crate::file::Process;

impl Process for RemoveOptions<'_> {
    fn process(&self, file: &mut String) {
        if self.first_n + self.last_n > 0 {
            self.first_last(file)
        }
        if 0 < self.from && self.from < file.len() && self.to > 0 {
            self.from_to(file)
        }

        if let Some(chars) = self.chars {
            self.remove_chars(file, chars);
        }
    }
}

impl RemoveOptions<'_> {
    fn first_last(&self, file: &mut String) {
        if self.first_n + self.last_n > file.len() {
            *file = "".to_owned();
        } else {
            let mut end = file.len() - self.last_n;
            if end < self.first_n {
                end = self.first_n;
            }
            *file = file[self.first_n..end].to_owned();
        }
    }

    fn from_to(&self, file: &mut String) {
        use std::cmp::min;
        // Change from 1 indexed to 0 indexed.
        for _ in (self.from - 1)..min(file.len(), self.to) {
            file.remove(self.from - 1);
        }
    }

    fn remove_chars(&self, file: &mut String, chars: &str) {
        // Change from 1 indexed to 0 indexed.
        for chr in chars.chars() {
            *file = file.replace(chr, "");
        }
    }
}

#[cfg(test)]
mod remove_tests {
    use super::*;

    #[test]
    fn test_first_n_chars() {
        let first_n = 5;
        let last_n = 0;
        let from = 0;
        let to = 0;
        let chars = None;
        let mut file = String::from("test_file");
        let opt = RemoveOptions {
            first_n,
            last_n,
            from,
            to,
            chars,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("file"))
    }

    #[test]
    fn test_last_n_chars() {
        let first_n = 0;
        let last_n = 5;
        let from = 0;
        let to = 0;
        let chars = None;
        let mut file = String::from("test_file");
        let opt = RemoveOptions {
            first_n,
            last_n,
            from,
            to,
            chars,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("test"))
    }

    #[test]
    fn test_first_and_last_n_chars() {
        let first_n = 4;
        let last_n = 4;
        let from = 0;
        let to = 0;
        let chars = None;
        let mut file = String::from("test_file");
        let opt = RemoveOptions {
            first_n,
            last_n,
            from,
            to,
            chars,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("_"))
    }

    #[test]
    fn test_too_many_removed_from_end() {
        let first_n = 6;
        let last_n = 4;
        let from = 0;
        let to = 0;
        let chars = None;
        let mut file = String::from("test_file");
        let opt = RemoveOptions {
            first_n,
            last_n,
            from,
            to,
            chars,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from(""))
    }

    #[test]
    fn test_too_many_removed_total() {
        let first_n = 60;
        let last_n = 4;
        let from = 0;
        let to = 0;
        let chars = None;
        let mut file = String::from("test_file");
        let opt = RemoveOptions {
            first_n,
            last_n,
            from,
            to,
            chars,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from(""))
    }

    #[test]
    fn test_from_to() {
        let first_n = 0;
        let last_n = 0;
        let from = 3;
        let to = 6;
        let chars = None;
        let mut file = String::from("test_file");
        let opt = RemoveOptions {
            first_n,
            last_n,
            from,
            to,
            chars,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("teile"))
    }
    #[test]
    fn test_from_to_one_character() {
        let first_n = 0;
        let last_n = 0;
        let from = 5;
        let to = 5;
        let chars = None;
        let mut file = String::from("test_file");
        let opt = RemoveOptions {
            first_n,
            last_n,
            from,
            to,
            chars,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("testfile"))
    }

    #[test]
    fn first_last_range() {
        let first_n = 2;
        let last_n = 2;
        let from = 2;
        let to = 4;
        let chars = None;
        let mut file = String::from("test_file");
        let opt = RemoveOptions {
            first_n,
            last_n,
            from,
            to,
            chars,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("si"))
    }

    #[test]
    fn remove_chars() {
        let first_n = 0;
        let last_n = 0;
        let from = 0;
        let to = 0;
        let chars = Some("ft");
        let mut file = String::from("test_file");
        let opt = RemoveOptions {
            first_n,
            last_n,
            from,
            to,
            chars,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("es_ile"))
    }
}
