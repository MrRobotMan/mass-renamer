use crate::{file::FileError, File};
use chrono::{DateTime, Local};
use egui::{Response, Sense, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use std::{
    cmp::Ordering,
    env,
    fmt::Display,
    fs::{canonicalize, read_dir},
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Default)]
pub struct Directory {
    files: Vec<(File, bool)>, // file and if it's slated to be changed
}

impl Directory {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DirectoryError> {
        let path = get_directory(path)?;
        let mut files = vec![];
        if let Some(p) = path.parent() {
            if let Ok(file) = p.try_into() {
                files.push((file, false))
            };
        };
        for p in read_dir(&path)? {
            if let Ok(file) = p?.path().try_into() {
                files.push((file, false));
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
    headers: [String; 6],
    sort_by: Columns,
    ascending: bool,
}

impl DirectoryView {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DirectoryError> {
        let directory = Directory::new(path)?;
        Ok(Self {
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

    fn sort(&mut self) {
        self.directory
            .files
            .sort_unstable_by(|lhs, rhs| match self.sort_by {
                Columns::Name => cmp(&lhs.0.original, &rhs.0.original),
                Columns::NewName => lhs.0.renamed.cmp(&rhs.0.renamed),
                Columns::Extension => lhs.0.extension.cmp(&rhs.0.extension),
                Columns::Size => lhs.0.info().3.cmp(&rhs.0.info().3),
                Columns::Modified => lhs.0.info().4.cmp(&rhs.0.info().4),
                Columns::Created => lhs.0.info().5.cmp(&rhs.0.info().5),
            });
        if !self.ascending {
            self.directory.files.reverse();
        };
    }
}

/// Custom ordering for files. Directories at the start or end.
fn cmp<T: AsRef<Path>, U: AsRef<Path>>(lhs: T, rhs: U) -> Ordering {
    match (lhs.as_ref().is_dir(), rhs.as_ref().is_dir()) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => rhs.as_ref().cmp(lhs.as_ref()),
    }
}

impl Widget for &mut DirectoryView {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            let headers = self.headers.clone();
            TableBuilder::new(ui)
                .column(Column::initial(240.))
                .column(Column::initial(240.))
                .column(Column::initial(60.))
                .column(Column::initial(60.))
                .column(Column::initial(240.))
                .column(Column::initial(240.))
                .resizable(true)
                .sense(Sense::click())
                .header(1.0, |mut head| {
                    for (idx, header) in headers.iter().enumerate() {
                        head.col(|ui| {
                            if ui.label(header).clicked() {
                                if idx == (self.sort_by as usize) {
                                    self.ascending = !self.ascending;
                                } else {
                                    self.ascending = false;
                                    self.sort_by = idx.into();
                                };
                                self.sort();
                            };
                        });
                    }
                })
                .body(|body| {
                    let items = &mut self.directory.files;
                    body.rows(30.0, items.len(), |mut row| {
                        let (item, selected) = &mut items[row.index()];
                        row.set_selected(*selected);
                        let info = item.info();
                        row.col(|ui| {
                            if ui
                                .label(if item.is_dir {
                                    format!("🗀 {}", info.0)
                                } else {
                                    info.0.to_string()
                                })
                                .clicked()
                            {
                                *selected = !*selected;
                            };
                        });
                        row.col(|ui| {
                            if ui.label(info.1).clicked() {
                                *selected = !*selected;
                            };
                        });
                        row.col(|ui| {
                            if ui.label(info.2.unwrap_or("")).clicked() {
                                *selected = !*selected;
                            };
                        });
                        row.col(|ui| {
                            if ui
                                .label(if let Some(size) = &info.3 {
                                    format!("{}", &size)
                                } else {
                                    String::new()
                                })
                                .clicked()
                            {
                                *selected = !*selected;
                            };
                        });
                        if let Some(time) = &info.4 {
                            row.col(|ui| {
                                if ui.label(datetime_to_string(time)).clicked() {
                                    *selected = !*selected;
                                };
                            });
                        };
                        if let Some(time) = &info.5 {
                            row.col(|ui| {
                                if ui.label(datetime_to_string(time)).clicked() {
                                    *selected = !*selected;
                                };
                            });
                        };
                        if row.response().clicked() {
                            *selected = !*selected;
                        }
                        if item.is_dir && row.response().double_clicked() {
                            // Change directory
                            todo!()
                        }
                    })
                });
        })
        .response
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
#[repr(usize)]
pub enum Columns {
    #[default]
    Name,
    NewName,
    Extension,
    Size,
    Modified,
    Created,
}

impl From<usize> for Columns {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Name,
            1 => Self::NewName,
            2 => Self::Extension,
            3 => Self::Size,
            4 => Self::Modified,
            5 => Self::Created,
            _ => unreachable!("Tried to convert {value}"),
        }
    }
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
