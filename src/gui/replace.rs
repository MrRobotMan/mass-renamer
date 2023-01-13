use egui::{Response, Ui, Widget};

#[derive(Default)]
pub struct ReplaceData {
    pub replace: String,
    pub with: String,
    pub match_case: bool,
}

pub struct ReplaceView<'a> {
    data: &'a mut ReplaceData,
}

impl<'a> ReplaceView<'a> {
    pub fn new(data: &'a mut ReplaceData) -> Self {
        Self { data }
    }
}

impl<'a> Widget for ReplaceView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.label("Replace");
            ui.horizontal(|ui| {
                ui.label("Replace: ");
                ui.text_edit_singleline(&mut self.data.replace);
            });
            ui.horizontal(|ui| {
                ui.label("With: ");
                ui.text_edit_singleline(&mut self.data.with);
            });
            ui.checkbox(&mut self.data.match_case, "Match Case")
        })
        .response
    }
}
