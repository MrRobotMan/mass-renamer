use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt::Display,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Local};
use egui::{Grid, Response, Ui, Widget};
use strum::{EnumCount, EnumIter};

use crate::File;

pub struct FileListing {
    pub name: PathBuf,
    pub renamed: File,
    pub extension: Option<String>,
    pub size: Option<u64>,
    pub modified: Option<DateTime<Local>>,
    pub created: Option<DateTime<Local>>,
    pub selected: bool,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, EnumIter, EnumCount)]
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

#[derive(Debug, Default)]
pub enum Order {
    #[default]
    Forward,
    Reverse,
}

pub struct FileView<'a> {
    files: &'a mut Vec<FileListing>,
    columns: &'a mut (Columns, Order, Columns),
    width: f32,
}

/// Return the datetime as a localized date and time.
fn datetime_to_string(datetime: &DateTime<Local>) -> String {
    format!("{}", datetime.format("%x %X"))
}

/// Show just the filename for a file
fn file_no_parents(path: &Path) -> Cow<'_, str> {
    match path.file_name() {
        None => Cow::Owned(String::new()),
        Some(file) => match path.is_dir() {
            false => file.to_string_lossy(),
            true => {
                let mut folder = String::from("🗀");
                folder.push_str(&file.to_string_lossy());
                Cow::Owned(folder)
            }
        },
    }
}

/// Custom ordering for files. Directories at the start or end.
fn cmp(rhs: &Path, lhs: &Path) -> Ordering {
    match (rhs.is_dir(), lhs.is_dir()) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => rhs.cmp(lhs),
    }
}

impl<'a> FileView<'a> {
    pub fn new(
        files: &'a mut Vec<FileListing>,
        columns: &'a mut (Columns, Order, Columns),
        width: f32,
    ) -> Self {
        Self {
            files,
            columns,
            width,
        }
    }
}

// TODO: Change to table!
impl<'a> Widget for FileView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        Grid::new("Files")
            .striped(true)
            .show(ui, |ui| {
                ui.set_width(self.width);
                ui.label("Sel");
                if ui
                    .selectable_value(&mut self.columns.0, Columns::Name, "Name")
                    .clicked()
                {
                    match self.columns {
                        (_, Order::Forward, Columns::Name) => {
                            self.files
                                .sort_unstable_by(|lhs, rhs| cmp(&rhs.name, &lhs.name));
                            self.columns.1 = Order::Reverse;
                        }
                        _ => {
                            self.files
                                .sort_unstable_by(|lhs, rhs| cmp(&lhs.name, &rhs.name));
                            self.columns.1 = Order::Forward;
                        }
                    };
                    self.columns.2 = Columns::Name;
                };
                if ui
                    .selectable_value(&mut self.columns.0, Columns::NewName, "New Name")
                    .clicked()
                {
                    match self.columns {
                        (_, Order::Forward, Columns::NewName) => {
                            self.files
                                .sort_unstable_by(|lhs, rhs| rhs.renamed.cmp(&lhs.renamed));
                            self.columns.1 = Order::Reverse;
                        }
                        _ => {
                            self.files
                                .sort_unstable_by(|lhs, rhs| lhs.renamed.cmp(&rhs.renamed));
                            self.columns.1 = Order::Forward;
                        }
                    };
                    self.columns.2 = Columns::NewName;
                };
                if ui
                    .selectable_value(&mut self.columns.0, Columns::Extension, "Type")
                    .clicked()
                {
                    match self.columns {
                        (_, Order::Forward, Columns::Extension) => {
                            self.files
                                .sort_unstable_by(|lhs, rhs| rhs.extension.cmp(&lhs.extension));
                            self.columns.1 = Order::Reverse;
                        }
                        _ => {
                            self.files
                                .sort_unstable_by(|lhs, rhs| lhs.extension.cmp(&rhs.extension));
                            self.columns.1 = Order::Forward;
                        }
                    };
                    self.columns.2 = Columns::Extension;
                };
                if ui
                    .selectable_value(&mut self.columns.0, Columns::Size, "Size")
                    .clicked()
                {
                    match self.columns {
                        (_, Order::Forward, Columns::Size) => {
                            self.files
                                .sort_unstable_by(|lhs, rhs| rhs.size.cmp(&lhs.size));
                            self.columns.1 = Order::Reverse;
                        }
                        _ => {
                            self.files
                                .sort_unstable_by(|lhs, rhs| lhs.size.cmp(&rhs.size));
                            self.columns.1 = Order::Forward;
                        }
                    };
                    self.columns.2 = Columns::Size;
                };
                if ui
                    .selectable_value(&mut self.columns.0, Columns::Modified, "Modified")
                    .clicked()
                {
                    match self.columns {
                        (_, Order::Forward, Columns::Modified) => {
                            self.files
                                .sort_unstable_by(|lhs, rhs| rhs.modified.cmp(&lhs.modified));
                            self.columns.1 = Order::Reverse;
                        }
                        _ => {
                            self.files
                                .sort_unstable_by(|lhs, rhs| lhs.modified.cmp(&rhs.modified));
                            self.columns.1 = Order::Forward;
                        }
                    };
                    self.columns.2 = Columns::Modified;
                };
                if ui
                    .selectable_value(&mut self.columns.0, Columns::Created, "Created")
                    .clicked()
                {
                    match self.columns {
                        (_, Order::Forward, Columns::Created) => {
                            self.files
                                .sort_unstable_by(|lhs, rhs| rhs.created.cmp(&lhs.created));
                            self.columns.1 = Order::Reverse;
                        }
                        _ => {
                            self.files
                                .sort_unstable_by(|lhs, rhs| lhs.created.cmp(&rhs.created));
                            self.columns.1 = Order::Forward;
                        }
                    };
                    self.columns.2 = Columns::Created;
                };
                ui.end_row();

                for item in self.files.iter_mut() {
                    ui.checkbox(&mut item.selected, "");
                    ui.label(file_no_parents(&item.name));
                    ui.label(&item.renamed);
                    ui.label(if let Some(ext) = &item.extension {
                        ext.as_str()
                    } else {
                        ""
                    });
                    ui.label(if let Some(size) = &item.size {
                        format!("{}", &size)
                    } else {
                        String::new()
                    });
                    if let Some(time) = &item.modified {
                        ui.label(datetime_to_string(time));
                    }
                    if let Some(time) = &item.created {
                        ui.label(datetime_to_string(time));
                    }
                    ui.end_row();
                }
            })
            .response
    }
}
