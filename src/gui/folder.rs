use super::{
    increment_decrement::{Arrows, Increment},
    valid_text::ValText,
    NUM_WIDTH,
};
use crate::FolderMode;
use egui::{Id, Response, Ui, Widget};

#[derive(Debug, Default)]
pub struct FolderData {
    pub position: FolderMode,
    pub sep: String,
    pub levels: ValText<i32>,
}
impl Increment for FolderData {
    fn increment(&mut self, increment: bool, _field: &str) {
        let delta = match increment {
            true => 1,
            false => -1,
        };
        if let Some(val) = self.levels.get_val() {
            self.levels.set_val(val + delta)
        } else if self.levels.is_empty() {
            self.levels.set_val(delta)
        }
    }
}
pub struct FolderView<'a> {
    data: &'a mut FolderData,
}

impl<'a> FolderView<'a> {
    pub fn new(data: &'a mut FolderData) -> Self {
        Self { data }
    }
}

impl<'a> Widget for FolderView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.label("Append Folder Name");
            ui.horizontal(|ui| {
                egui::ComboBox::new("Append File Name", "")
                    .selected_text(format!("{:?}", &self.data.position))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.data.position, FolderMode::None, "None");
                        ui.selectable_value(&mut self.data.position, FolderMode::Prefix, "Prefix");
                        ui.selectable_value(&mut self.data.position, FolderMode::Suffix, "Suffix")
                    });
                ui.label("Sep.");
                ui.text_edit_singleline(&mut self.data.sep);
                ui.separator();
                ui.label("Pos.");
                if ui
                    .add(egui::TextEdit::singleline(&mut self.data.levels).desired_width(NUM_WIDTH))
                    .changed()
                    && !self.data.levels.is_valid()
                {
                    self.data.levels.revert();
                };
                ui.add(Arrows {
                    id: Id::new("Folder Arrows"),
                    value: self.data,
                    field: "folder",
                });
            });
        })
        .response
    }
}
