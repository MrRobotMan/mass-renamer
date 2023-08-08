use std::{error::Error, fmt::Write, path::Path, time::SystemTime};

use super::{File, Process};
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
#[derive(Default, Debug, Clone)]
pub struct DateOptions {
    date_mode: DateMode,
    date_type: DateType,
    fmt: DateFormat,
    sep: String,
    seg: String,
    full_year: bool,
}

impl Process for DateOptions {
    fn process(&self, file: &mut File) {
        if let Ok(datetime) = self.get_date(&file.original) {
            let format = match &self.fmt {
                DateFormat::Std((prefix, suffix)) => {
                    let mut fmt = prefix.get_format(&self.seg, self.full_year);
                    if let Some(suf) = suffix {
                        fmt.push_str(&self.seg);
                        fmt.push_str(&suf.get_format(&self.seg));
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
                    write!(file.stem, "{}{}", self.sep, datetime.format(&format))
                        .expect("Unexpected error appending to string.");
                }
            }
        }
    }
}

impl DateOptions {
    fn get_date(&self, file: &Path) -> Result<DateTime<Local>, Box<dyn Error>> {
        let metadata = file.metadata()?;
        let dt = match self.date_type {
            DateType::Created => metadata.created()?,
            DateType::Modified => metadata.modified()?,
            DateType::Current => SystemTime::now(),
        };
        let datetime: DateTime<Local> = dt.into();
        Ok(datetime)
    }
}

/// Select from
/// `DateMode::Prefix` or
/// `DateMode::Suffix`.
#[derive(Default, PartialEq, Debug, Clone)]
pub enum DateMode {
    #[default]
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
#[derive(Default, PartialEq, Debug, Clone)]
pub enum DateType {
    #[default]
    Created,
    Modified,
    Current,
}

/// Select from
/// - `DateFormat::Std(DatePrefix, Option<DateSuffix>)` to use the standard options
/// - `DateFormat::Custom(&str)` to use a custom `strftime` format
#[derive(Debug, Clone)]
pub enum DateFormat {
    Std((DatePrefix, Option<DateSuffix>)),
    Custom(String),
}

impl Default for DateFormat {
    fn default() -> Self {
        Self::Std((DatePrefix::Dmy, None))
    }
}

/// Select from
/// - `DatePrefix::DMY` for Day Month Year
/// - `DatePrefix::MDY` for Month Year Day
/// - `DatePrefix::YMD` for Year Month Day
#[derive(Default, Debug, Clone)]
pub enum DatePrefix {
    #[default]
    Dmy,
    Mdy,
    Ymd,
}

impl DatePrefix {
    fn get_format(&self, sep: &str, full_year: bool) -> String {
        let y = if full_year { "%Y" } else { "%y" };
        match self {
            Self::Dmy => format!("%d{sep}%m{sep}{y}"),
            Self::Mdy => format!("%m{sep}%d{sep}{y}"),
            Self::Ymd => format!("{y}{sep}%m{sep}%d"),
        }
    }
}

/// Select from
/// - `DateSuffix::HM` for Hour Minute
/// - `DateSuffix::HMS` for Hour Minute Second
#[derive(Debug, Clone)]
pub enum DateSuffix {
    Hm,
    Hms,
}

impl DateSuffix {
    fn get_format(&self, sep: &str) -> String {
        match self {
            Self::Hm => format!("%H{sep}%M{sep}"),
            Self::Hms => format!("%H{sep}%M{sep}%S"),
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
            let mut file = File::new(Path::new("test file.txt")).unwrap();
            let date_mode = DateMode::Prefix;
            let date_type = DateType::Modified;
            let fmt = DateFormat::Std((DatePrefix::Dmy, None));
            let sep = "-".into();
            let seg = "_".into();
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
            let mut file = File::new(Path::new("test file.txt")).unwrap();
            let date_mode = DateMode::Suffix;
            let date_type = DateType::Created;
            let fmt = DateFormat::Std((DatePrefix::Dmy, Some(DateSuffix::Hms)));
            let sep = "".into();
            let seg = "_".into();
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
            let mut file = File::new(Path::new("test file.txt")).unwrap();
            let date_mode = DateMode::Prefix;
            let date_type = DateType::Current;
            let fmt = DateFormat::Custom("%v++".into());
            let sep = "~".into();
            let seg = "_".into();
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
