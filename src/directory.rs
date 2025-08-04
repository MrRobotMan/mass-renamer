use crate::{renamer::FileError, File};
use chrono::{DateTime, Local};
use egui::{Response, Sense, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use std::{
    cmp::Ordering,
    env,
    fmt::Display,
    fs::{canonicalize, read_dir},
    iter::Zip,
    path::{Path, PathBuf},
    slice::Iter,
};
use thiserror::Error;

#[derive(Default)]
pub struct Directory {
    files: Vec<File>,
}

impl Directory {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DirectoryError> {
        let path = get_directory(path)?;
        let mut files = vec![];
        if let Some(p) = path.parent() {
            if let Ok(file) = p.try_into() {
                files.push(file)
            };
        };
        for p in read_dir(&path)? {
            if let Ok(file) = p?.path().try_into() {
                files.push(file);
            }
        }
        Ok(Self { files })
    }
}

/// Get the full path of a directory falling back to the home directory
/// if nothing is provided. If the provided path is a file, the file's parent
/// is returned.
pub fn get_initial_directory<P: AsRef<Path>>(path: Option<P>) -> Result<PathBuf, DirectoryError> {
    if let Some(arg) = path {
        get_directory(arg)
    } else {
        match env::current_dir() {
            Ok(dir) => Ok(dir),
            Err(_) => {
                let d = home::home_dir();
                match d {
                    Some(path) => Ok(path),
                    None => Err(DirectoryError::NoHome),
                }
            }
        }
    }
}

/// Get the full path of a directory.
/// If the provided path is a file, the file's parent is returned.
pub fn get_directory<P: AsRef<Path>>(path: P) -> Result<PathBuf, DirectoryError> {
    let p = canonicalize(path)?;
    if p.is_file() {
        Ok(p.parent().unwrap().into())
    } else {
        Ok(p)
    }
}

#[derive(Debug, Error)]
pub enum DirectoryError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("No home directory could be found")]
    NoHome,
    #[error(transparent)]
    File(#[from] FileError),
}

#[derive(Default)]
pub struct DirectoryView {
    directory: Directory,
    selected: Vec<bool>,
    headers: [String; 6],
    sort_by: Columns,
    ascending: bool,
}

impl DirectoryView {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DirectoryError> {
        let directory = Directory::new(path)?;
        Ok(Self {
            selected: vec![false; directory.files.len()],
            directory,
            headers: [
                "Name".into(),
                "New Name".into(),
                "Type".into(),
                "Size".into(),
                "Modified".into(),
                "Created".into(),
            ],
            ..Default::default()
        })
    }

    pub fn files(&self) -> Zip<Iter<bool>, Iter<File>> {
        self.selected.iter().zip(self.directory.files.iter())
    }


impl Widget for &mut DirectoryView {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            TableBuilder::new(ui)
                .columns(Column::auto(), self.headers.len())
                .resizable(true)
                // .striped(true)
                .sense(Sense::click())
                .header(10.0, |mut header| {
                    for c in self.headers.iter() {
                        header.col(|ui| {
                            ui.heading(c.to_string());
                        });
                    }
                })
                .body(|mut body| {
                    for item in self.directory.files.iter_mut() {
                        // ui.checkbox(&mut item.selected, "");
                        body.row(30.0, |mut row| {
                            let info = item.info();
                            row.col(|ui| {
                                ui.label(info.0);
                            });
                            row.col(|ui| {
                                ui.label(info.1);
                            });
                            row.col(|ui| {
                                ui.label(info.2.unwrap_or(""));
                            });
                            row.col(|ui| {
                                ui.label(if let Some(size) = &info.3 {
                                    format!("{}", &size)
                                } else {
                                    String::new()
                                });
                            });
                            if let Some(time) = &info.4 {
                                row.col(|ui| {
                                    ui.label(datetime_to_string(time));
                                });
                            }
                            if let Some(time) = &info.5 {
                                row.col(|ui| {
                                    ui.label(datetime_to_string(time));
                                });
                            }
                        })
                    }
                });
        })
        .response
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Columns {
    #[default]
    Name,
    NewName,
    Extension,
    Size,
    Created,
    Modified,
}

impl Display for Columns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Columns::Name => "Name",
            Columns::NewName => "New Name",
            Columns::Extension => "Ext",
            Columns::Size => "Type",
            Columns::Created => "Created",
            Columns::Modified => "Modified",
        })
    }
}

/// Return the datetime as a localized date and time.
fn datetime_to_string(datetime: &DateTime<Local>) -> String {
    format!("{}", datetime.format("%x %X"))
}
