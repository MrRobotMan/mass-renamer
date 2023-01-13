use std::{
    cmp::Ordering,
    fs,
    path::{Path, PathBuf},
};

use crate::*;
use eframe::{self, App, CreationContext};
use egui::{
    self, menu, style::Margin, Align, CentralPanel, Color32, Context, Frame, Key, Layout, RichText,
    Rounding, ScrollArea, Stroke, TextEdit, TopBottomPanel, Visuals, WidgetText,
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

use add::{AddData, AddView};
use case::{CaseData, CaseView};
use date::{DateData, DateView};
use extension::{ExtensionData, ExtensionView};
use files::{Columns, FileListing, FileView, Order};
use folder::{FolderData, FolderView};
use name::{NameData, NameView};
use number::{NumberData, NumberView};
use reg::{RegExData, RegExView};
use remove::{RemoveData, RemoveView};
use replace::{ReplaceData, ReplaceView};

const FILES_HEIGHT: f32 = 300.0;
const FRAME_RADIUS: f32 = 10.0;
const FRAME_MARGIN: f32 = 5.0;
const NUM_WIDTH: f32 = 15.0;

#[derive(Default)]
pub struct Renamer<'a> {
    cwd: String,
    cwd_path: PathBuf,
    files: Vec<FileListing>,
    columns: (Columns, Order, Columns), // 3rd field is previous
    add: AddData,
    case: CaseData,
    date: DateData<'a>,
    extension: ExtensionData,
    folder: FolderData,
    name: NameData,
    number: NumberData,
    reg_exp: RegExData,
    remove: RemoveData,
    replace: ReplaceData,
}

#[allow(clippy::from_over_into)]
impl Into<WidgetText> for &RenameFile {
    fn into(self) -> WidgetText {
        WidgetText::RichText(RichText::new(match &self.extension {
            None => self.stem.clone(),
            Some(ext) => format!("{}.{}", &self.stem, ext),
        }))
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

impl Renamer<'_> {
    //! Called once before the first frame.
    pub fn new(cc: &CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(Visuals::dark());
        let mut app: Renamer = Default::default();
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
                if let Ok(meta) = fs::metadata(file.path()) {
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
                    file_listing.push(FileListing {
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

fn frame() -> Frame {
    Frame::none()
        .stroke(Stroke::new(1.0, Color32::BLACK))
        .inner_margin(Margin::same(FRAME_MARGIN))
        .rounding(Rounding::same(FRAME_RADIUS))
}

impl App for Renamer<'_> {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // let Self { label, value } = self;

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            // Status bar.
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label("Status: Ready");
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
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
                    let response =
                        ui.add_sized(ui.available_size(), TextEdit::singleline(&mut self.cwd));
                    if response.lost_focus() && ui.input().key_pressed(Key::Enter) {
                        self.change_dir()
                    };
                });
                ScrollArea::vertical()
                    .max_height(FILES_HEIGHT)
                    .show(ui, |ui| {
                        ui.add(FileView::new(&mut self.files, &mut self.columns))
                    });
                ui.horizontal(|ui| {
                    // ui.with_layout(Layout::top_down_justified(Align::Center),
                    ui.vertical(|ui| {
                        frame().show(ui, |ui| ui.add(RegExView::new(&mut self.reg_exp)));
                        frame().show(ui, |ui| ui.add(NameView::new(&mut self.name)));
                        frame().show(ui, |ui| ui.add(FolderView::new(&mut self.folder)));
                    });
                    ui.vertical(|ui| {
                        frame().show(ui, |ui| ui.add(ReplaceView::new(&mut self.replace)));
                        frame().show(ui, |ui| ui.add(CaseView::new(&mut self.case)));
                        frame().show(ui, |ui| ui.add(ExtensionView::new(&mut self.extension)));
                    });
                    frame().show(ui, |ui| ui.add(RemoveView::new(&mut self.remove)));
                    frame().show(ui, |ui| ui.add(AddView::new(&mut self.add)));
                    frame().show(ui, |ui| ui.label("Auto Date"));
                    frame().show(ui, |ui| ui.label("Numbering"));
                });
            })
        });
    }
}
