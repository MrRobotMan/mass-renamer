use std::{
    borrow::Cow,
    cmp::Ordering,
    fs,
    path::{Path, PathBuf},
};

use crate::RenameFile;
use chrono::{DateTime, Local};
use eframe;
use egui::{self, WidgetText};
use home;
use rfd;

mod data;

use data::*;

type FileListing = (
    PathBuf,
    RenameFile,
    Option<u64>,
    Option<DateTime<Local>>,
    Option<DateTime<Local>>,
    bool, // is selected
);

const FILES_HEIGHT: (f32, f32) = (100.0, 300.0);

#[derive(Default)]
pub struct App<'a> {
    cwd: String,
    cwd_path: PathBuf,
    files: Vec<FileListing>,
    columns: (Columns, Order, Columns), // 3rd field is previous
    _add: AddData,
    _case: CaseData,
    _date: DateData<'a>,
    _extension: ExtensionData<'a>,
    _folder: Folderdata,
    _name: NameData<'a>,
    _number: Numberdata,
    _reg_exp: RegExData,
    _remove: RemoveData,
    _replace: ReplaceData,
}

#[allow(clippy::from_over_into)]
impl Into<WidgetText> for &RenameFile {
    fn into(self) -> WidgetText {
        WidgetText::RichText(egui::widget_text::RichText::new(match &self.extension {
            None => self.stem.clone(),
            Some(ext) => format!("{}.{}", &self.stem, ext),
        }))
    }
}

/// Return the datetime as a localized date and time.
fn datetime_to_string(datetime: &DateTime<Local>) -> String {
    format!("{}", datetime.format("%x %X"))
}

/// Show just the filename for a file
fn file_no_parents(path: &Path) -> Cow<'_, str> {
    match path.file_name() {
        None => Cow::Owned(String::new()),
        Some(file) => file.to_string_lossy(),
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

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
enum Columns {
    #[default]
    Name,
    NewName,
    Size,
    Created,
    Modified,
}
#[derive(Debug, Default)]
enum Order {
    #[default]
    Forward,
    Reverse,
}

impl App<'_> {
    //! Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let mut app: App = Default::default();
        let cwd_path = match home::home_dir() {
            Some(dir) => dir,
            None => PathBuf::default(),
        };
        app.cwd_path = cwd_path.clone();
        app.cwd = cwd_path.display().to_string();
        app.file_list();
        app
    }
    fn change_dir(&mut self) {
        self.cwd_path = PathBuf::from(&self.cwd);
        self.file_list();
    }

    fn up_one(&mut self) {
        if let Some(dir) = self.cwd_path.parent() {
            self.cwd_path = PathBuf::from(dir);
            self.cwd = self.cwd_path.display().to_string();
            self.file_list();
        };
    }

    fn file_list(&mut self) {
        if let Ok(dir) = self.cwd_path.read_dir() {
            let mut file_listing = Vec::new();
            for file in dir.flatten() {
                let file_name = file.path();
                let renamed = RenameFile::new(&file.path());
                let mut size = None;
                let mut modified = None;
                let mut created = None;
                if let Ok(meta) = fs::metadata(&file.path()) {
                    #[cfg(windows)]
                    if format!("{:?}", meta.file_type()).contains("attributes: 38") {
                        continue; // Remove system hidden files (.blf, .regtrans-ms, etc)
                    }
                    println!("{file_name:?}");
                    if file_name.is_file() {
                        size = Some(meta.len())
                    };
                    if let Ok(dt) = meta.modified() {
                        modified = Some(dt.into());
                    };
                    if let Ok(dt) = meta.created() {
                        created = Some(dt.into());
                    };
                }
                if let Some(renamed) = renamed {
                    file_listing.push((file_name, renamed, size, modified, created, false));
                }
            }
            file_listing.sort_unstable_by(|lhs, rhs| cmp(&lhs.0, &rhs.0));
            self.files = file_listing;
        }
    }

    fn _process_selected(&mut self) {
        for (_cnt, file) in self.files.iter().enumerate() {
            if file.5 {
                let mut _orig = &file.0;
                let mut _renamed = &file.1;
                // self.add.make_options().process(&mut renamed);
            }
        }
    }
}

impl eframe::App for App<'_> {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let Self { label, value } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            // Status bar.
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("Status: Ready");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    if ui.small_button("Select Folder").clicked() {
                        if let Some(dir) = rfd::FileDialog::new()
                            .set_directory(&self.cwd_path)
                            .pick_folder()
                        {
                            self.cwd = dir.display().to_string();
                            self.change_dir();
                        }
                    };
                    if ui.small_button("up").clicked() {
                        self.up_one();
                    };
                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut self.cwd),
                    );
                    if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                        self.change_dir()
                    };
                });
                egui::ScrollArea::vertical()
                    .max_height(FILES_HEIGHT.1)
                    .show(ui, |ui| {
                        egui::Grid::new("Files").striped(true).show(ui, |ui| {
                            ui.label("Sel");
                            if ui
                                .selectable_value(&mut self.columns.0, Columns::Name, "Name")
                                .clicked()
                            {
                                match self.columns {
                                    (_, Order::Forward, Columns::Name) => {
                                        self.files.sort_unstable_by(|lhs, rhs| cmp(&rhs.0, &lhs.0));
                                        self.columns.1 = Order::Reverse;
                                    }
                                    _ => {
                                        self.files.sort_unstable_by(|lhs, rhs| cmp(&lhs.0, &rhs.0));
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
                                        self.files.sort_unstable_by(|lhs, rhs| rhs.1.cmp(&lhs.1));
                                        self.columns.1 = Order::Reverse;
                                    }
                                    _ => {
                                        self.files.sort_unstable_by(|lhs, rhs| lhs.1.cmp(&rhs.1));
                                        self.columns.1 = Order::Forward;
                                    }
                                };
                                self.columns.2 = Columns::NewName;
                            };
                            if ui
                                .selectable_value(&mut self.columns.0, Columns::Size, "Size")
                                .clicked()
                            {
                                match self.columns {
                                    (_, Order::Forward, Columns::Size) => {
                                        self.files.sort_unstable_by(|lhs, rhs| rhs.2.cmp(&lhs.2));
                                        self.columns.1 = Order::Reverse;
                                    }
                                    _ => {
                                        self.files.sort_unstable_by(|lhs, rhs| lhs.2.cmp(&rhs.2));
                                        self.columns.1 = Order::Forward;
                                    }
                                };
                                self.columns.2 = Columns::Size;
                            };
                            if ui
                                .selectable_value(
                                    &mut self.columns.0,
                                    Columns::Modified,
                                    "Modified",
                                )
                                .clicked()
                            {
                                match self.columns {
                                    (_, Order::Forward, Columns::Modified) => {
                                        self.files.sort_unstable_by(|lhs, rhs| rhs.3.cmp(&lhs.3));
                                        self.columns.1 = Order::Reverse;
                                    }
                                    _ => {
                                        self.files.sort_unstable_by(|lhs, rhs| lhs.3.cmp(&rhs.3));
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
                                        self.files.sort_unstable_by(|lhs, rhs| rhs.4.cmp(&lhs.4));
                                        self.columns.1 = Order::Reverse;
                                    }
                                    _ => {
                                        self.files.sort_unstable_by(|lhs, rhs| lhs.4.cmp(&rhs.4));
                                        self.columns.1 = Order::Forward;
                                    }
                                };
                                self.columns.2 = Columns::Created;
                            };
                            ui.end_row();

                            for item in self.files.iter_mut() {
                                ui.checkbox(&mut item.5, "");
                                ui.label(file_no_parents(&item.0));
                                ui.label(&item.1);
                                ui.label(if let Some(size) = &item.2 {
                                    format!("{}", &size)
                                } else {
                                    String::new()
                                });
                                if let Some(time) = &item.3 {
                                    ui.label(datetime_to_string(time));
                                }
                                if let Some(time) = &item.4 {
                                    ui.label(datetime_to_string(time));
                                }
                                ui.end_row();
                            }
                        });
                    });
                ui.horizontal(|ui| ui.label("This will be the fields area."));
            })
        });
    }
}
