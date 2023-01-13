use std::{
    borrow::Cow,
    cmp::Ordering,
    fs,
    path::{Path, PathBuf},
};

use crate::*;
use chrono::{DateTime, Local};
use eframe;
use egui::{
    self, style::Margin, Color32, Frame, InnerResponse, Response, Rounding, Stroke, Ui, Widget,
    WidgetText,
};
use home;
use rfd;

mod add;
mod case;
mod date;
mod extension;
mod files;
mod folder;
mod increment_decrement;
mod name;
mod number;
mod reg;
mod remove;
mod replace;
mod valid_text;

/*
let num_less_than_ten = ValText::with_validator(|text| {
  text.parse().ok().filter(|&n| n < 10)
});

ui.text_edit_singleline(&mut num_less_than_ten); */

const FILES_HEIGHT: f32 = 300.0;
const FRAME_RADIUS: f32 = 10.0;
const FRAME_MARGIN: f32 = 5.0;
const NUM_WIDTH: f32 = 15.0;

#[derive(Default)]
pub struct App<'a> {
    cwd: String,
    cwd_path: PathBuf,
    files: Vec<files::FileListing>,
    columns: (files::Columns, files::Order, files::Columns), // 3rd field is previous
    _add: add::AddData,
    case: case::CaseData,
    _date: date::DateData<'a>,
    extension: extension::ExtensionData,
    folder: folder::FolderData,
    name: name::NameData,
    _number: number::NumberData,
    reg_exp: reg::RegExData,
    remove: remove::RemoveData,
    replace: replace::ReplaceData,
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
                let name = file.path();
                let extension = name
                    .extension()
                    .map(|ext| ext.to_string_lossy().to_string());
                let renamed = RenameFile::new(&file.path());
                let mut size = None;
                let mut modified = None;
                let mut created = None;
                if let Ok(meta) = fs::metadata(&file.path()) {
                    #[cfg(windows)]
                    if format!("{:?}", meta.file_type()).contains("attributes: 38") {
                        continue; // Remove system hidden files (.blf, .regtrans-ms, etc)
                    }
                    if name.is_file() {
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
                    file_listing.push(files::FileListing {
                        name,
                        renamed,
                        extension,
                        size,
                        modified,
                        created,
                        selected: false,
                    });
                }
            }
            file_listing.sort_unstable_by(|lhs, rhs| cmp(&lhs.name, &rhs.name));
            self.files = file_listing;
        }
    }

    fn _process_selected(&mut self) {
        for (_cnt, file) in self.files.iter().enumerate() {
            if file.selected {
                let mut _orig = &file.name;
                let mut _renamed = &file.renamed;
                // self.add.make_options().process(&mut renamed);
            }
        }
    }
}

fn framed_widget(ui: &mut Ui, widget: impl Widget) -> InnerResponse<Response> {
    Frame::none()
        .stroke(Stroke::new(1.0, Color32::BLACK))
        .inner_margin(Margin::same(FRAME_MARGIN))
        .rounding(Rounding::same(FRAME_RADIUS))
        .show(ui, |ui| ui.add(widget))
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
                    .max_height(FILES_HEIGHT)
                    .show(ui, |ui| {
                        ui.add(files::FileView::new(&mut self.files, &mut self.columns))
                    });
                ui.horizontal(|ui| {
                    // ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center),
                    ui.vertical(|ui| {
                        framed_widget(ui, reg::RegExView::new(&mut self.reg_exp));
                        framed_widget(ui, name::NameView::new(&mut self.name));
                        framed_widget(ui, folder::FolderView::new(&mut self.folder));
                    });
                    ui.vertical(|ui| {
                        framed_widget(ui, replace::ReplaceView::new(&mut self.replace));
                        framed_widget(ui, case::CaseView::new(&mut self.case));
                        framed_widget(ui, extension::ExtensionView::new(&mut self.extension));
                    });
                    framed_widget(ui, remove::RemoveView::new(&mut self.remove));
                    Frame::none()
                        .stroke(Stroke::new(1.0, Color32::BLACK))
                        .inner_margin(Margin::same(FRAME_MARGIN))
                        .rounding(Rounding::same(FRAME_RADIUS))
                        .show(ui, |ui| ui.label("Add"));
                    Frame::none()
                        .stroke(Stroke::new(1.0, Color32::BLACK))
                        .inner_margin(Margin::same(FRAME_MARGIN))
                        .rounding(Rounding::same(FRAME_RADIUS))
                        .show(ui, |ui| ui.label("Auto Date"));
                    Frame::none()
                        .stroke(Stroke::new(1.0, Color32::BLACK))
                        .inner_margin(Margin::same(FRAME_MARGIN))
                        .rounding(Rounding::same(FRAME_RADIUS))
                        .show(ui, |ui| ui.label("Numbering"));
                });
            })
        });
    }
}
