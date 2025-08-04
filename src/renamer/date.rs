use egui::{ComboBox, Response, TextEdit, Ui, Widget};
use std::{error::Error, path::Path, time::SystemTime};

use super::{OptionBuilder, Process, Renamer};
use chrono::{DateTime, Local};

/// Use the prefix or suffix `Mode` to modify the filename with a date format.
/// The `Date` that the file was created, modified, or the current date can be added in
/// the format (`FMT`) selected. A `Sep`erator can be specified for the character(s)
/// between the filename and the date as well as a format for setting the character(s)
/// between date `Seg`ments. Select the `YYYY` box to display years as 4 digit instead
/// of the default 2 (except for custom dates).
///
/// You also have the option to specify your own custom date formats using
/// [chrono::format::strftime](https://docs.rs/chrono/0.4.31/chrono/format/strftime/index.html) specifiers.
#[derive(Default, Debug, Clone)]
pub struct DateOptions {
    date_mode: DateMode,
    date_type: DateType,
    fmt: DateFormat,
    custom_fmt: String,
    sep: String,
    seg: String,
    full_year: bool,
}

impl Process for DateOptions {
    fn process(&self, file: &mut Renamer) {
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
                DateFormat::Custom => self.custom_fmt.clone(),
            };
            match self.date_mode {
                DateMode::Prefix => file
                    .stem
                    .insert_str(0, &format!("{}{}", datetime.format(&format), self.sep)),
                DateMode::Suffix => {
                    file.stem
                        .push_str(&format!("{}{}", self.sep, datetime.format(&format)));
                }
                DateMode::None => {}
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
/// `DateMode::Prefix`,
/// `DateMode::Suffix`.
#[derive(Default, PartialEq, Debug, Clone)]
pub enum DateMode {
    Prefix,
    Suffix,
    #[default]
    None,
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
/// - `DateFormat::Custom` to use a custom `strftime` format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateFormat {
    Std((DatePrefix, Option<DateSuffix>)),
    Custom,
}

impl Default for DateFormat {
    fn default() -> Self {
        Self::Std((DatePrefix::Dmy, None))
    }
}

impl DateFormat {
    fn format(&self) -> &str {
        match self {
            Self::Std((DatePrefix::Dmy, None)) => "DMY",
            Self::Std((DatePrefix::Mdy, None)) => "MDY",
            Self::Std((DatePrefix::Ymd, None)) => "YMD",
            Self::Std((DatePrefix::Dmy, Some(DateSuffix::Hm))) => "DMY HM",
            Self::Std((DatePrefix::Mdy, Some(DateSuffix::Hm))) => "MDY HM",
            Self::Std((DatePrefix::Ymd, Some(DateSuffix::Hm))) => "YMD HM",
            Self::Std((DatePrefix::Dmy, Some(DateSuffix::Hms))) => "DMY HMS",
            Self::Std((DatePrefix::Mdy, Some(DateSuffix::Hms))) => "MDY HMS",
            Self::Std((DatePrefix::Ymd, Some(DateSuffix::Hms))) => "YMD HMS",
            Self::Custom => "Custom",
        }
    }

    fn iter() -> impl Iterator<Item = DateFormat> {
        [
            Self::Std((DatePrefix::Dmy, None)),
            Self::Std((DatePrefix::Mdy, None)),
            Self::Std((DatePrefix::Ymd, None)),
            Self::Std((DatePrefix::Dmy, Some(DateSuffix::Hm))),
            Self::Std((DatePrefix::Mdy, Some(DateSuffix::Hm))),
            Self::Std((DatePrefix::Ymd, Some(DateSuffix::Hm))),
            Self::Std((DatePrefix::Dmy, Some(DateSuffix::Hms))),
            Self::Std((DatePrefix::Mdy, Some(DateSuffix::Hms))),
            Self::Std((DatePrefix::Ymd, Some(DateSuffix::Hms))),
            Self::Custom,
        ]
        .iter()
        .copied()
    }
}

/// Select from
/// - `DatePrefix::DMY` for Day Month Year
/// - `DatePrefix::MDY` for Month Year Day
/// - `DatePrefix::YMD` for Year Month Day
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateSuffix {
    Hm,
    Hms,
}

impl DateSuffix {
    fn get_format(&self, sep: &str) -> String {
        match self {
            Self::Hm => format!("%H{sep}%M"),
            Self::Hms => format!("%H{sep}%M{sep}%S"),
        }
    }
}

#[derive(Default)]
pub struct DateView {
    data: DateOptions,
    width: f32,
}

impl DateView {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }
}

impl OptionBuilder for DateView {
    type Processor = DateOptions;

    fn build(&self) -> DateOptions {
        self.data.clone()
    }
}

impl Widget for &mut DateView {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_width(self.width);
            ui.label("Date");
            ui.horizontal(|ui| {
                ui.set_width(self.width);
                ui.label("Mode");
                ComboBox::from_id_source("Date Mode")
                    .selected_text(match self.data.date_mode {
                        DateMode::Prefix => "Prefix",
                        DateMode::Suffix => "Suffix",
                        DateMode::None => "None",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.data.date_mode, DateMode::None, "None");
                        ui.selectable_value(&mut self.data.date_mode, DateMode::Prefix, "Prefix");
                        ui.selectable_value(&mut self.data.date_mode, DateMode::Suffix, "Suffix");
                    });
            });
            ui.horizontal(|ui| {
                ui.set_width(self.width);
                ui.label("Type");
                ComboBox::from_id_source("Date Type")
                    .selected_text(match self.data.date_type {
                        DateType::Created => "Created",
                        DateType::Modified => "Modified",
                        DateType::Current => "Now",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.data.date_type, DateType::Created, "Created");
                        ui.selectable_value(
                            &mut self.data.date_type,
                            DateType::Modified,
                            "Modified",
                        );
                        ui.selectable_value(&mut self.data.date_type, DateType::Current, "Now");
                    });
            });
            ui.horizontal(|ui| {
                ui.set_width(self.width);
                ui.label("Format");
                if ComboBox::from_id_source("Date Fmt")
                    .selected_text(self.data.fmt.format())
                    .show_ui(ui, |ui| {
                        for opt in DateFormat::iter() {
                            ui.selectable_value(&mut self.data.fmt, opt, opt.format());
                        }
                    })
                    .response
                    .changed()
                    && self.data.fmt != DateFormat::Custom
                {
                    self.data.custom_fmt = String::new();
                };
            });

            ui.horizontal(|ui| {
                ui.set_width(self.width);
                ui.label("Custom");
                if ui.text_edit_singleline(&mut self.data.custom_fmt).changed()
                    && !self.data.custom_fmt.is_empty()
                {
                    self.data.fmt = DateFormat::Custom;
                };
            });
            ui.horizontal(|ui| {
                ui.set_width(self.width);
                ui.label("Sep.");
                ui.add(TextEdit::singleline(&mut self.data.sep).desired_width(30.0));
                ui.label("Seg");
                ui.add(TextEdit::singleline(&mut self.data.seg).desired_width(30.0));
            });
            ui.checkbox(&mut self.data.full_year, "4 Digit Year");
        })
        .response
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
            let mut file = Renamer::new(Path::new("test file.txt")).unwrap();
            let date_mode = DateMode::Prefix;
            let date_type = DateType::Modified;
            let fmt = DateFormat::Std((DatePrefix::Dmy, None));
            let custom_fmt = String::new();
            let sep = "-".into();
            let seg = "_".into();
            let full_year = true;
            let opt = DateOptions {
                date_mode,
                date_type,
                fmt,
                custom_fmt,
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
        run_test(&vec!["test file.txt"], || {
            let mut file = Renamer::new(Path::new("test file.txt")).unwrap();
            let date_mode = DateMode::Suffix;
            let date_type = DateType::Created;
            let fmt = DateFormat::Std((DatePrefix::Dmy, Some(DateSuffix::Hm)));
            let custom_fmt = String::new();
            let sep = "".into();
            let seg = "_".into();
            let full_year = false;
            let opt = DateOptions {
                date_mode,
                date_type,
                fmt,
                custom_fmt,
                sep,
                seg,
                full_year,
            };
            opt.process(&mut file);
            let date = format!("{}", chrono::Local::now().format("%d_%m_%y_%H_%M"));
            let expected = format!("test file{date}");
            assert_eq!(file.stem, expected);
        })
    }

    #[test]
    fn prefix_date_current_custom_format() {
        crate::tester::run_test(&vec!["test file.txt"], || {
            let mut file = Renamer::new(Path::new("test file.txt")).unwrap();
            let date_mode = DateMode::Prefix;
            let date_type = DateType::Current;
            let fmt = DateFormat::Custom;
            let custom_fmt = String::from("%v++");
            let sep = "~".into();
            let seg = "_".into();
            let full_year = true;
            let opt = DateOptions {
                date_mode,
                date_type,
                fmt,
                custom_fmt,
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
