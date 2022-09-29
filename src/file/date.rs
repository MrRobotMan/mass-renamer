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
        let datetime = self.get_date(&file.original);
        if datetime.is_err() {
            return;
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

pub enum DateMode {
    Before,
    After,
}

pub enum DateType {
    Created,
    Modified,
    Current,
}

pub enum DateFormat<'a> {
    Std(StdDateFormat),
    Custom(&'a str),
}

pub struct StdDateFormat {
    pub prefix: DatePrefix,
    pub suffix: Option<DateSuffix>,
}

pub enum DatePrefix {
    DMY,
    MDY,
    YMD,
}
pub enum DateSuffix {
    HM,
    HMS,
}

#[cfg(test)]
mod date_tests {
    use super::*;
    use std::{fs, panic, path::Path};

    #[allow(unused_must_use)]
    fn run_test<T>(test: T) -> ()
    where
        T: FnOnce() -> () + panic::UnwindSafe,
    {
        fs::File::create("test file.txt");
        let result = panic::catch_unwind(|| test());
        fs::remove_file("test file.txt");
        assert!(result.is_ok())
    }

    #[test]
    fn prefix_date_created_hyphen_separator_full_year() {
        run_test(|| {
            let mut file = RenameFile::new(Path::new("test file.txt")).unwrap();
            let date_mode = DateMode::Before;
            let date_type = DateType::Created;
            let fmt = DateFormat::Std(StdDateFormat {
                prefix: DatePrefix::DMY,
                suffix: None,
            });
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
            let date = format!("{}", chrono::Local::now().format("%d_%m_%y"));
            let expected = format!("{date}-test file");
            opt.process(&mut file);
            assert_eq!(file.stem, expected);
        })
    }
}
