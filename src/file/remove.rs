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
/// - `High` - Remove ASCII characters (chars from 128 to 255).
/// - `Trim` - Remove leading and trailing spaces.
/// - `D/S` - Remove occurrences of double spaces, and replace them with single spaces.
/// - `Chars` - Remove all characters (matching regex a-zA-Z).
/// - `Sym` - Remove all symbols (~`!@#$%^&*()_-+={}[]|\/?"':;.,<>).
/// - `Lead Dots` - Remove ".", "..", or both from the front of filenames.
///
/// Note: When you use the `words` option, you have the ability of specifying a special
/// value using the wildcard (*). This will remove the specified string, and any
/// characters occupied by the wildcard. So for example, specifying [*] would convert
/// "Hello[ABC] Joe" to just "Hello Joe", as it has removed the two square brackets and
/// everything between. The wildcard can not be at the start or end of the word.
/// For that case use crop.

pub struct RemoveOptions<'a> {
    first_n: usize,
    last_n: usize,
    range: (usize, usize),
    characters: Option<&'a str>,
    words: Option<&'a str>,
    crop: Option<(bool, &'a str)>,
    digits: bool,
    ascii_high: bool,
    trim: bool,
    double_space: bool,
    chars: bool,
    symbols: bool,
    lead_dots: LeadDots,
}

pub enum LeadDots {
    None,
    One,
    Two,
    Both,
}

use crate::file::Process;

impl Process for RemoveOptions<'_> {
    fn process(&self, file: &mut String) {
        if self.first_n + self.last_n > 0 {
            self.first_last(file)
        }
        if 0 < self.range.0 && self.range.0 < file.len() && self.range.1 > 0 {
            self.from_to(file)
        }

        if let Some(characters) = self.characters {
            for chr in characters.chars() {
                self.remove_char(file, chr);
            }
        }

        if let Some(words) = self.words {
            for word in words.split(" ") {
                self.remove_word(file, word);
            }
        }

        if let Some((before, position)) = self.crop {
            let pos = file.find(position);
            match (before, pos) {
                (true, Some(p)) => *file = file[p..].to_owned(),
                (false, Some(p)) => *file = file[..(p + position.len())].to_owned(),
                _ => (),
            }
        }

        if self.digits {
            for chr in "01233456789".chars() {
                self.remove_char(file, chr);
            }
        }

        if self.ascii_high {
            let chars = (128..=255).map(|c| char::from(c));
            for chr in chars {
                self.remove_char(file, chr);
            }
        }

        if self.trim {
            *file = file.trim().to_owned();
        }

        if self.chars {
            let chars = (65..=90)
                .map(|c| char::from(c))
                .chain((97..=122).map(|c| char::from(c)));
            for chr in chars {
                self.remove_char(file, chr)
            }
        }

        if self.symbols {
            for chr in r#"~`!@#$%^&*()_-+={}[]|\/?"':;.,<>"#.chars() {
                self.remove_char(file, chr)
            }
        }

        match self.lead_dots {
            LeadDots::None => (),
            LeadDots::One => {
                if file.starts_with(".") && !file.starts_with("..") {
                    file.remove(0);
                }
            }
            LeadDots::Two => {
                if file.starts_with("..") {
                    *file = file[2..].to_owned()
                }
            }
            LeadDots::Both => {
                while file.starts_with(".") {
                    file.remove(0);
                }
            }
        }

        if self.double_space {
            self.remove_double_spaces(file)
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
        for _ in (self.range.0 - 1)..min(file.len(), self.range.1) {
            file.remove(self.range.0 - 1);
        }
    }

    fn remove_char(&self, file: &mut String, chr: char) {
        *file = file.replace(chr, "");
    }

    fn remove_word(&self, file: &mut String, word: &str) {
        if word.contains("*") {
            let w = word.split("*").collect::<Vec<&str>>();
            let (start, end) = (w[0], w[1]);
            let start_idx = file.find(start);
            let end_idx = file.find(end);
            match (start_idx, end_idx) {
                (Some(start_idx), Some(end_idx)) => {
                    let word = &file[start_idx..(end_idx + end.len())];
                    *file = file.replace(word, "");
                }
                _ => return (),
            }
        } else {
            *file = file.replace(&word, "")
        }
    }

    fn remove_double_spaces(&self, file: &mut String) {
        while file.contains("  ") {
            *file = file.replace("  ", " ");
        }
    }
}

#[cfg(test)]
mod remove_tests {
    use super::*;

    #[test]
    fn combined_removals() {
        let first_n = 2;
        let last_n = 2;
        let range = (1, 2);
        let characters = Some("ft");
        let words = Some("ile w*h");
        let crop = None;
        let digits = true;
        let ascii_high = true;
        let trim = true;
        let double_space = true;
        let chars = false;
        let symbols = true;
        let lead_dots = LeadDots::None;
        let mut file = String::from("some test file  1234withÃ!  testing");
        let opt = RemoveOptions {
            first_n,
            last_n,
            range,
            characters,
            words,
            crop,
            digits,
            ascii_high,
            trim,
            double_space,
            chars,
            symbols,
            lead_dots,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("es esi"))
    }

    #[test]
    fn test_too_many_removed_from_end() {
        let first_n = 6;
        let last_n = 4;
        let range = (0, 0);
        let characters = None;
        let words = None;
        let crop = None;
        let digits = false;
        let ascii_high = false;
        let trim = false;
        let double_space = false;
        let chars = false;
        let symbols = false;
        let lead_dots = LeadDots::None;
        let mut file = String::from("test_file");
        let opt = RemoveOptions {
            first_n,
            last_n,
            range,
            characters,
            words,
            crop,
            digits,
            ascii_high,
            trim,
            double_space,
            chars,
            symbols,
            lead_dots,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from(""))
    }

    #[test]
    fn test_too_many_removed_total() {
        let first_n = 60;
        let last_n = 4;
        let range = (0, 0);
        let characters = None;
        let words = None;
        let crop = None;
        let digits = false;
        let ascii_high = false;
        let trim = false;
        let double_space = false;
        let chars = false;
        let symbols = false;
        let lead_dots = LeadDots::None;
        let mut file = String::from("test_file");
        let opt = RemoveOptions {
            first_n,
            last_n,
            range,
            characters,
            words,
            crop,
            digits,
            ascii_high,
            trim,
            double_space,
            chars,
            symbols,
            lead_dots,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from(""))
    }

    #[test]
    fn crop_before() {
        let first_n = 0;
        let last_n = 0;
        let range = (0, 0);
        let characters = None;
        let words = None;
        let crop = Some((true, "to"));
        let digits = false;
        let ascii_high = false;
        let trim = false;
        let double_space = false;
        let chars = false;
        let symbols = false;
        let lead_dots = LeadDots::One;
        let mut file = String::from("file to test");
        let opt = RemoveOptions {
            first_n,
            last_n,
            range,
            characters,
            words,
            crop,
            digits,
            ascii_high,
            trim,
            double_space,
            chars,
            symbols,
            lead_dots,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("to test"));
    }

    #[test]
    fn remove_chars_lead_dot() {
        let first_n = 0;
        let last_n = 0;
        let range = (0, 0);
        let characters = None;
        let words = None;
        let crop = None;
        let digits = false;
        let ascii_high = false;
        let trim = false;
        let double_space = false;
        let chars = true;
        let symbols = false;
        let lead_dots = LeadDots::One;
        let mut file = String::from(".file123");
        let mut file2 = String::from("..file123");
        let opt = RemoveOptions {
            first_n,
            last_n,
            range,
            characters,
            words,
            crop,
            digits,
            ascii_high,
            trim,
            double_space,
            chars,
            symbols,
            lead_dots,
        };
        opt.process(&mut file);
        opt.process(&mut file2);
        assert_eq!(file, String::from("123"));
        assert_eq!(file2, String::from("..123"));
    }

    #[test]
    fn crop_after_found_lead_dots() {
        let first_n = 0;
        let last_n = 0;
        let range = (0, 0);
        let characters = None;
        let words = None;
        let crop = Some((false, "file"));
        let digits = false;
        let ascii_high = false;
        let trim = false;
        let double_space = false;
        let chars = false;
        let symbols = false;
        let lead_dots = LeadDots::Two;
        let mut file = String::from(".file123");
        let mut file2 = String::from("..file123");
        let opt = RemoveOptions {
            first_n,
            last_n,
            range,
            characters,
            words,
            crop,
            digits,
            ascii_high,
            trim,
            double_space,
            chars,
            symbols,
            lead_dots,
        };
        opt.process(&mut file);
        opt.process(&mut file2);
        assert_eq!(file, String::from(".file"));
        assert_eq!(file2, String::from("file"));
    }

    #[test]
    fn remove_words_not_found_all_lead_dots() {
        let first_n = 0;
        let last_n = 0;
        let range = (0, 0);
        let characters = None;
        let words = Some("test word");
        let crop = None;
        let digits = false;
        let ascii_high = false;
        let trim = false;
        let double_space = false;
        let chars = false;
        let symbols = false;
        let lead_dots = LeadDots::Both;
        let mut file = String::from(".file123");
        let mut file2 = String::from("..file123");
        let opt = RemoveOptions {
            first_n,
            last_n,
            range,
            characters,
            words,
            crop,
            digits,
            ascii_high,
            trim,
            double_space,
            chars,
            symbols,
            lead_dots,
        };
        opt.process(&mut file);
        opt.process(&mut file2);
        assert_eq!(file, String::from("file123"));
        assert_eq!(file2, String::from("file123"));
    }
}
