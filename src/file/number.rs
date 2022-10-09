use crate::file::{Process, RenameFile};
use std::fmt::Write;

/// Add sequential numbers to the file.
/// - `Mode` - Choose between prefix, suffix, both, or insert at a location (0 indexed).
/// - `Value` - Specify a value number for the numbering.
/// - `Step` - The number to be added to the previous.
/// - `Pad` - The minimum number of digits occupied by the numeric element.
/// - `Char` - The character to use for padding. By default, numeric bases will be padded with leading zeros; the a-z and A-Z options will be padded with "a" or "A" as appropriate.
/// - `Sep`. - A character or characters that you wish to be inserted between the old filename and the number. If you enter the special character ":" (colon) in the Sep. box then this will be replaced with the auto-number. So a separator value of ABC:DEF: would result in ABC1DEF1, ABC2ABC2 etc.
/// - `Format` - You can choose to append the auto-number in any various bases (binary, decimal, hex (upper and lower), octal), ASCII letters A-Z.
pub struct NumberingOptions<'a> {
    pub mode: NumberMode,
    pub value: u32,
    pub pad: usize,
    pub char: char,
    pub sep: &'a str,
    pub format: NumberFormat,
}

impl Process for NumberingOptions<'_> {
    fn process(&self, file: &mut RenameFile) {
        let val = self.number_value();
        match self.mode {
            NumberMode::Prefix => file.stem.insert_str(0, &format!("{}{}", val, self.sep)),
            NumberMode::Suffix => write!(file.stem, "{}{}", self.sep, val)
                .expect("Unexpected error appending string."),
            NumberMode::Insert(idx) => file
                .stem
                .insert_str(idx, &format!("{}{}{}", self.sep, val, self.sep)),
        };
    }
}

impl NumberingOptions<'_> {
    fn number_value(&self) -> String {
        let replace = match &self.format {
            NumberFormat::Decimal => format!("{}", self.value),
            NumberFormat::Binary => format!("{:b}", self.value),
            NumberFormat::Octal => format!("{:o}", self.value),
            NumberFormat::Hex_Upper => format!("{:X}", self.value),
            NumberFormat::Hex_Lower => format!("{:x}", self.value),
            f => {
                let offset = match f {
                    NumberFormat::ASCII_Lower => 96_u8,
                    _ => 64_u8,
                };
                let mut res: Vec<char> = Vec::new();
                let mut val = self.value;
                while val > 0 {
                    res.push(char::from((val % 26) as u8 + offset));
                    val /= 26;
                }
                res.reverse();
                res.into_iter().collect::<String>()
            }
        };
        if self.pad > replace.len() {
            let mut val = std::iter::repeat(self.char)
                .take(self.pad - replace.len())
                .collect::<String>();
            val.push_str(&replace);
            val
        } else {
            replace
        }
    }
}

/// Select from
/// `NumberMode::Prefix`,
/// `NumberMode::Suffix`, or
/// `NumberMode::Insert(usize)`.
pub enum NumberMode {
    Prefix,
    Suffix,
    Insert(usize),
}

/// Select from
/// `NumberFormat:Binary`,
/// `NumberFormat:Decimal`,
/// `NumberFormat:Hex_Upper`,
/// `NumberFormat:Hex_Lower`,
/// `NumberFormat:Octal`,
/// `NumberFormat:ASCII_Upper`, or
/// `NumberFormat:ASCII_Lower`
#[allow(non_camel_case_types)]
pub enum NumberFormat {
    Binary,
    Decimal,
    Hex_Upper,
    Hex_Lower,
    Octal,
    ASCII_Upper,
    ASCII_Lower,
}

#[cfg(test)]
mod numbering_test {
    use super::*;
    use std::path::Path;

    fn vec_compare(va: &[String], vb: &[String]) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| (a == b))
    }

    #[test]
    fn prefix_decimal_with_padding() {
        let mut files = (0..10)
            .map(|_| RenameFile::new(Path::new("TestFile.txt")).unwrap())
            .collect::<Vec<RenameFile>>();
        let pad = 2;
        let char = '0';
        let sep = "--";
        for (value, file) in files.iter_mut().enumerate() {
            let format = NumberFormat::Decimal;
            let mode = NumberMode::Prefix;
            let opt = NumberingOptions {
                mode,
                value: (&value + 1) as u32,
                pad,
                char,
                sep,
                format,
            };
            opt.process(file);
        }
        let expected = (1..=10)
            .map(|i| format!("{i:02}--TestFile"))
            .collect::<Vec<String>>();
        let result = files
            .iter()
            .map(|f| f.stem.clone())
            .collect::<Vec<String>>();
        assert!(vec_compare(&result, &expected));
    }

    #[test]
    fn suffix_binary_no_padding() {
        let mut file = RenameFile::new(Path::new("TestFile.txt")).unwrap();
        let format = NumberFormat::Binary;
        let value = 5;
        let pad = 0;
        let char = '0';
        let sep = ".";
        let mode = NumberMode::Suffix;
        let opt = NumberingOptions {
            mode,
            value,
            pad,
            char,
            sep,
            format,
        };
        opt.process(&mut file);
        assert_eq!(file.stem, "TestFile.101");
    }

    #[test]
    fn insert_ascii_upper() {
        let mut file = RenameFile::new(Path::new("TestFile.txt")).unwrap();
        let format = NumberFormat::ASCII_Upper;
        let value = 50;
        let pad = 0;
        let char = '0';
        let sep = "_";
        let mode = NumberMode::Insert(4);
        let opt = NumberingOptions {
            mode,
            value,
            pad,
            char,
            sep,
            format,
        };
        opt.process(&mut file);
        assert_eq!(file.stem, "Test_AX_File");
    }
}
