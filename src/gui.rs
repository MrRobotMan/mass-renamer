use std::{fs, path::PathBuf};

use crate::RenameFile;
use chrono::{DateTime, Local};
use eframe;
use egui::{self, Color32, Frame, Stroke, WidgetText};
use home;
use rfd;

mod data;

use data::*;

type FileListing = (
    PathBuf,
    RenameFile,
    u64,
    Option<DateTime<Local>>,
    Option<DateTime<Local>>,
);

#[derive(Default)]
pub struct App<'a> {
    cwd: String,
    cwd_path: PathBuf,
    files: Vec<FileListing>,
    columns: Columns,
    _add: data::AddData,
    _case: data::CaseData,
    _date: data::DateData<'a>,
    _extension: data::ExtensionData<'a>,
    _folder: data::Folderdata,
    _name: data::NameData<'a>,
    _number: data::Numberdata,
    _reg_exp: data::RegExData,
    _remove: data::RemoveData,
    _replace: data::ReplaceData,
    selected: Vec<usize>,
}

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

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
enum Columns {
    #[default]
    Name,
    NewName,
    Size,
    Created,
    Modified,
}

impl App<'_> {
    //! Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(egui::Visuals::light());
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
            let mut dir_listing = Vec::new();
            for file in dir.flatten() {
                let file_name = file.path();
                let renamed = RenameFile::new(&file.path());
                let mut size = 0;
                let mut modified = None;
                let mut created = None;
                if let Ok(meta) = fs::metadata(&file.path()) {
                    size = meta.len();
                    if let Ok(dt) = meta.modified() {
                        modified = Some(dt.into());
                    };
                    if let Ok(dt) = meta.created() {
                        created = Some(dt.into());
                    };
                }
                if let Some(renamed) = renamed {
                    dir_listing.push((file_name, renamed, size, modified, created));
                }
            }
            self.files = dir_listing;
        }
    }

    fn process_selected(&mut self) {
        for (_cnt, file) in self.selected.iter().enumerate() {
            let mut _orig = &self.files[*file].0;
            let mut _renamed = &self.files[*file].1;
            // self.add.make_options().process(&mut renamed);
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
                egui::Grid::new("Files").striped(true).show(ui, |ui| {
                    Frame::none()
                        .stroke(Stroke::new(1.0, Color32::BLACK))
                        .show(ui, |ui| {
                            ui.selectable_value(&mut self.columns, Columns::Name, "Name")
                        });
                    Frame::none()
                        .stroke(Stroke::new(1.0, Color32::BLACK))
                        .show(ui, |ui| {
                            ui.selectable_value(&mut self.columns, Columns::NewName, "New Name")
                        });
                    Frame::none()
                        .stroke(Stroke::new(1.0, Color32::BLACK))
                        .show(ui, |ui| {
                            ui.selectable_value(&mut self.columns, Columns::Size, "Size")
                        });
                    Frame::none()
                        .stroke(Stroke::new(1.0, Color32::BLACK))
                        .show(ui, |ui| {
                            ui.selectable_value(&mut self.columns, Columns::Created, "Created")
                        });
                    Frame::none()
                        .stroke(Stroke::new(1.0, Color32::BLACK))
                        .show(ui, |ui| {
                            ui.selectable_value(&mut self.columns, Columns::Modified, "Modified")
                        });
                    ui.end_row();

                    for file in self.files.iter() {
                        Frame::none()
                            .stroke(Stroke::new(1.0, Color32::BLACK))
                            .show(ui, |ui| ui.label(file.0.to_string_lossy()));
                        Frame::none()
                            .stroke(Stroke::new(1.0, Color32::BLACK))
                            .show(ui, |ui| ui.label(&file.1));
                        Frame::none()
                            .stroke(Stroke::new(1.0, Color32::BLACK))
                            .show(ui, |ui| ui.label(format!("{}", &file.2)));
                        if let Some(time) = file.3 {
                            Frame::none()
                                .stroke(Stroke::new(1.0, Color32::BLACK))
                                .show(ui, |ui| ui.label(datetime_to_string(&time)));
                        }
                        if let Some(time) = file.4 {
                            Frame::none()
                                .stroke(Stroke::new(1.0, Color32::BLACK))
                                .show(ui, |ui| ui.label(datetime_to_string(&time)));
                        }
                        ui.end_row();
                    }
                });
                ui.horizontal(|ui| ui.label("This will be the fields area."));
            })
        });

        // egui::SidePanel::left("side_panel").show(ctx, |ui| {
        //     ui.heading("Side Panel");

        //     ui.horizontal(|ui| {
        //         ui.label("Write something: ");
        //         ui.text_edit_singleline(label);
        //     });

        //     ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
        //     if ui.button("Increment").clicked() {
        //         *value += 1.0;
        //     }

        //     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //         ui.horizontal(|ui| {
        //             ui.spacing_mut().item_spacing.x = 0.0;
        //             ui.label("powered by ");
        //             ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        //             ui.label(" and ");
        //             ui.hyperlink_to(
        //                 "eframe",
        //                 "https://github.com/emilk/egui/tree/master/crates/eframe",
        //             );
        //             ui.label(".");
        //         });
        //     });
        // });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.heading("eframe template");
        //     ui.hyperlink("https://github.com/emilk/eframe_template");
        //     ui.add(egui::github_link_file!(
        //         "https://github.com/emilk/eframe_template/blob/master/",
        //         "Source code."
        //     ));
        //     egui::warn_if_debug_build(ui);
        // });
    }
}
