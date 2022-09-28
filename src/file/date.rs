use crate::file::{Process, RenameFile};
use chrono;
use std::{fs, path::Path};

/// Use the prefix or suffix `Mode` to modify the filename with a date format.
/// The `Date` that the file was created, modified, or the current date can be added in
/// the format (`FMT`) selected. A `Sep`erator can be specified for the character(s)
/// between the filename and the date as well as a format for setting the character(s)
/// between date `Seg`ments. Select the `YYYY` box to display years as 4 digit instead
/// of the default 2.
///
/// You also have the option to specify your own custom date formats using
/// [chrono::format::strftime](https://docs.rs/chrono/0.4.20/chrono/format/strftime/index.html) specifiers.
pub struct DateOptions {}

impl Process for DateOptions {
    fn process(&self, file: &mut RenameFile) {}
}

#[cfg(test)]
mod date_tests {

    #[test]
    fn prefix_date() {}
}
