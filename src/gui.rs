use std::path::PathBuf;

use eframe::{
    egui::{
        menu, Align, CentralPanel, Color32, Context, Frame, Key, Layout, Margin, Rounding, Stroke,
        TextEdit, TopBottomPanel, Visuals,
    },
    run_native, App, CreationContext, NativeOptions, Theme,
};
use egui::ViewportCommand;

use crate::renamer::{
    add::AddView, case::CaseView, date::DateView, directory::DirectoryView,
    extension::ExtensionView, folder::FolderView, name::NameView, number::NumberView,
    reg::RegexView, remove::RemoveView, replace::ReplaceView,
};

mod increment_decrement;
mod valid_text;

pub use increment_decrement::{Arrows, Incrementer};
pub use valid_text::ValText;

const FRAME_MARGIN: f32 = 5.0;
const FRAME_RADIUS: f32 = 10.0;
// const FILES_WIDTH: f32 = 1200.0;
pub const NUM_WIDTH: f32 = 15.0;
const COL_WIDTH: f32 = 450.0;

pub fn run() -> eframe::Result<()> {
    let native_options = NativeOptions {
        centered: true,
        follow_system_theme: true,
        default_theme: Theme::Dark,
        ..Default::default()
    };
    run_native(
        "Bulk Renamer",
        native_options,
        Box::new(|cc| Box::new(Renamer::new(cc))),
    )
}

#[derive(Default)]
pub struct Renamer {
    cwd: String,
    cwd_path: PathBuf,
    files: DirectoryView,
    add: AddView,
    case: CaseView,
    date: DateView,
    extension: ExtensionView,
    folder: FolderView,
    name: NameView,
    number: NumberView,
    reg_exp: RegexView,
    remove: RemoveView,
    replace: ReplaceView,
}

impl Renamer {
    //! Called once before the first frame.
    pub fn new(cc: &CreationContext) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(Visuals::dark());
        let cwd_path = home::home_dir().unwrap_or_default();
        let files = DirectoryView::new(&cwd_path).unwrap();
        let cwd = cwd_path.display().to_string();

        Self {
            cwd_path,
            cwd,
            files,
            reg_exp: RegexView::new(COL_WIDTH),
            name: NameView::new(COL_WIDTH),
            folder: FolderView::new(COL_WIDTH),
            replace: ReplaceView::new(COL_WIDTH),
            case: CaseView::new(COL_WIDTH / 2.0),
            extension: ExtensionView::new(COL_WIDTH / 2.0),
            remove: RemoveView::new(COL_WIDTH / 2.0),
            add: AddView::new(COL_WIDTH / 2.0),
            date: DateView::new(COL_WIDTH / 2.0),
            number: NumberView::new(COL_WIDTH / 2.0),
        }
    }

    fn change_dir(&mut self) {
        self.files = DirectoryView::new(&self.cwd).unwrap();
    }

    fn up_one(&mut self) {
        if let Some(dir) = self.cwd_path.parent() {
            self.cwd_path = PathBuf::from(dir);
            self.cwd = self.cwd_path.display().to_string();
            self.files = DirectoryView::new(&self.cwd).unwrap();
        };
    }

    // fn file_list(&mut self) {
    //     if let Ok(dir) = self.cwd_path.read_dir() {
    //         let mut file_listing = Vec::new();
    //         for file in dir.flatten() {
    //             let name = file.path();
    //             let extension = name
    //                 .extension()
    //                 .map(|ext| ext.display());
    //             let renamed = File::try_from(&file.path());
    //             let mut size = None;
    //             let mut modified = None;
    //             let mut created = None;
    //             if let Ok(meta) = fs::metadata(file.path()) {
    //                 #[cfg(windows)]
    //                 if format!("{:?}", meta.file_type()).contains("attributes: 38") {
    //                     continue; // Remove system hidden files (.blf, .regtrans-ms, etc)
    //                 }
    //                 if name.is_file() {
    //                     size = Some(meta.len())
    //                 };
    //                 if let Ok(dt) = meta.modified() {
    //                     modified = Some(dt.into());
    //                 };
    //                 if let Ok(dt) = meta.created() {
    //                     created = Some(dt.into());
    //                 };
    //             }
    //             if let Ok(renamed) = renamed {
    //                 file_listing.push(FileListing {
    //                     name,
    //                     renamed,
    //                     extension,
    //                     size,
    //                     modified,
    //                     created,
    //                     selected: false,
    //                 });
    //             }
    //         }
    //         file_listing.sort_unstable_by(|lhs, rhs| cmp(&lhs.name, &rhs.name));
    //         self.files = file_listing;
    //     }
    // }

    fn _process_selected(&mut self) {
        todo!()
        // for (_cnt, (selected, _file)) in self.files.files().enumerate() {
        // if *selected {
        // let mut _orig = file.name;
        // let mut _renamed = file.renamed;
        // self.add.make_options().process(&mut renamed);
        // }
        // }
    }
}

fn frame() -> Frame {
    Frame::none()
        .stroke(Stroke::new(1.0, Color32::BLACK))
        .inner_margin(Margin::same(FRAME_MARGIN))
        .rounding(Rounding::same(FRAME_RADIUS))
}

impl App for Renamer {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Menu bar.
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close)
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
                    if response.lost_focus() && ui.input(|inp| inp.key_pressed(Key::Enter)) {
                        self.change_dir()
                    };
                });
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        frame().show(ui, |ui| ui.add(&mut self.reg_exp));
                        frame().show(ui, |ui| ui.add(&mut self.name));
                        frame().show(ui, |ui| ui.add(&mut self.folder));
                        frame().show(ui, |ui| ui.add(&mut self.replace));
                        ui.horizontal(|ui| {
                            frame().show(ui, |ui| ui.add(&mut self.case));
                            frame().show(ui, |ui| ui.add(&mut self.extension));
                        });
                        ui.horizontal(|ui| {
                            frame().show(ui, |ui| ui.add(&mut self.remove));
                            frame().show(ui, |ui| ui.add(&mut self.number));
                        });
                        ui.horizontal(|ui| {
                            frame().show(ui, |ui| ui.add(&mut self.date));
                            frame().show(ui, |ui| ui.add(&mut self.add));
                        });
                    });
                    ui.add_space(FRAME_MARGIN);
                    frame().show(ui, |ui| ui.add(&mut self.files));
                    // frame().show(ui, |ui| {
                    //     ScrollArea::vertical().show(ui, |ui| {
                    //         ui.vertical(|ui| {
                    //             ui.add(FileView::new(
                    //                 &mut self.files,
                    //                 &mut self.columns,
                    //                 FILES_WIDTH,
                    //             ))
                    //         });
                    //     });
                    // });
                });
            });
        });

        // Set the window size
        ctx.send_viewport_cmd(ViewportCommand::InnerSize(ctx.used_size()));
    }
}
