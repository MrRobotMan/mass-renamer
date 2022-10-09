use std::{error::Error, path::Path, time::SystemTime};

use crate::file::{Process, RenameFile};
use chrono::{DateTime, Local};

/// Use the prefix or suffix `Mode` to modify the filename with a date format.
/// The `Date` that the file was created, modified, or the current date can be added in
/// the format (`FMT`) selected. A `Sep`erator can be specified for the character(s)
/// between the filename and the date as well as a format for setting the character(s)
/// between date `Seg`ments. Select the `YYYY` box to display years as 4 digit instead
/// of the default 2 (except for custom dates).
///
/// You also have the option to specify your own custom date formats using
/// [chrono::format::strftime](https://docs.rs/chrono/0.4.20/chrono/format/strftime/index.html) specifiers.
pub struct DateOptions<'a> {
    date_mode: DateMode,
    date_type: DateType,
    fmt: DateFormat<'a>,
    sep: &'a str,
    seg: &'a str,
    full_year: bool,
}

impl Process for DateOptions<'_> {
    fn process(&self, file: &mut RenameFile) {
        if let Ok(datetime) = self.get_date(&file.original) {
            let format = match &self.fmt {
                DateFormat::Std((prefix, suffix)) => {
                    let mut fmt = prefix.get_format(self.seg, self.full_year);
                    if let Some(suf) = suffix {
                        fmt.push_str(self.seg);
                        fmt.push_str(&suf.get_format(self.seg));
                    }
                    fmt
                }
                DateFormat::Custom(fmt) => (*fmt).to_owned(),
            };
            match self.date_mode {
                DateMode::Prefix => file
                    .stem
                    .insert_str(0, &format!("{}{}", datetime.format(&format), self.sep)),
                DateMode::Suffix => {
                    file.stem
                        .push_str(&format!("{}{}", self.sep, datetime.format(&format)));
                }
            }
        }
    }
}

impl DateOptions<'_> {
    fn get_date(&self, file: &Path) -> Result<DateTime<Local>, Box<dyn Error>> {
        let metadata = file.metadata()?;
        let dt = match self.date_type {
            DateType::Created => metadata.created()?,
            DateType::Modified => metadata.modified()?,
            DateType::Current => SystemTime::now(),
        };
        let datetime: DateTime<Local> = dt.clone().into();
        Ok(datetime)
    }
}

/// Select from
/// `DateMode::Prefix` or
/// `DateMode::Suffix`.
pub enum DateMode {
    Prefix,
    Suffix,
}

/// Select from
/// - `DateType::Created` for the file creation date
/// - `Datetype::Modified` for the date last modified
/// - `DateType::Current` for today's date
///
/// Note, if an OS does not support `Created` or `Modified` this option will
/// result in no change to the file name.
pub enum DateType {
    Created,
    Modified,
    Current,
}

/// Select from
/// - `DateFormat::Std(DatePrefix, Option<DateSuffix>)` to use the standard options
/// - `DateFormat::Custom(&str)` to use a custom `strftime` format
pub enum DateFormat<'a> {
    Std((DatePrefix, Option<DateSuffix>)),
    Custom(&'a str),
}

/// Select from
/// - `DatePrefix::DMY` for Day Month Year
/// - `DatePrefix::MDY` for Month Year Day
/// - `DatePrefix::YMD` for Year Month Day
pub enum DatePrefix {
    DMY,
    MDY,
    YMD,
}

impl DatePrefix {
    fn get_format(&self, sep: &str, full_year: bool) -> String {
        let y = if full_year { "%Y" } else { "%y" };
        match self {
            DatePrefix::DMY => format!("%d{sep}%m{sep}{y}"),
            DatePrefix::MDY => format!("%m{sep}%d{sep}{y}"),
            DatePrefix::YMD => format!("{y}{sep}%m{sep}%d"),
        }
    }
}

/// Select from
/// - `DateSuffix::HM` for Hour Minute
/// - `DateSuffix::HMS` for Hour Minute Second
pub enum DateSuffix {
    HM,
    HMS,
}

impl DateSuffix {
    fn get_format(&self, sep: &str) -> String {
        match self {
            DateSuffix::HM => format!("%H{sep}%M{sep}"),
            DateSuffix::HMS => format!("%H{sep}%M{sep}%S"),
        }
    }
}

#[cfg(test)]
mod date_tests {
    use super::*;
    use crate::tester::run_test;
    use std::path::Path;

    #[test]
    fn prefix_date_modified_hyphen_separator_full_year() {
        run_test(&vec!["test file.txt"], || {
            let mut file = RenameFile::new(Path::new("test file.txt")).unwrap();
            let date_mode = DateMode::Prefix;
            let date_type = DateType::Modified;
            let fmt = DateFormat::Std((DatePrefix::DMY, None));
            let sep = "-";
            let seg = "_";
            let full_year = true;
            let opt = DateOptions {
                date_mode,
                date_type,
                fmt,
                sep,
                seg,
                full_year,
            };
            let date = format!("{}", chrono::Local::now().format("%d_%m_%Y"));
            let expected = format!("{date}-test file");
            opt.process(&mut file);
            assert_eq!(file.stem, expected);
        })
    }

    #[test]
    fn suffix_date_created_no_separator() {
        crate::tester::run_test(&vec!["test file.txt"], || {
            let date = format!("{}", chrono::Local::now().format("%d_%m_%y_%H_%M_%S"));
            let mut file = RenameFile::new(Path::new("test file.txt")).unwrap();
            let date_mode = DateMode::Suffix;
            let date_type = DateType::Created;
            let fmt = DateFormat::Std((DatePrefix::DMY, Some(DateSuffix::HMS)));
            let sep = "";
            let seg = "_";
            let full_year = false;
            let opt = DateOptions {
                date_mode,
                date_type,
                fmt,
                sep,
                seg,
                full_year,
            };
            let expected = format!("test file{date}");
            opt.process(&mut file);
            assert_eq!(file.stem, expected);
        })
    }

    #[test]
    fn prefix_date_current_custom_format() {
        crate::tester::run_test(&vec!["test file.txt"], || {
            let mut file = RenameFile::new(Path::new("test file.txt")).unwrap();
            let date_mode = DateMode::Prefix;
            let date_type = DateType::Current;
            let fmt = DateFormat::Custom("%v++");
            let sep = "~";
            let seg = "_";
            let full_year = true;
            let opt = DateOptions {
                date_mode,
                date_type,
                fmt,
                sep,
                seg,
                full_year,
            };
            let date = format!("{}", chrono::Local::now().format("%v"));
            let expected = format!("{date}++~test file");
            opt.process(&mut file);
            assert_eq!(file.stem, expected);
        })
    }
}
