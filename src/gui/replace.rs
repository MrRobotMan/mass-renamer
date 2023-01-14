use egui::{Response, Ui, Widget};

#[derive(Default)]
pub struct ReplaceData {
    replace: String,
    with: String,
    match_case: bool,
}

pub struct ReplaceView<'a> {
    data: &'a mut ReplaceData,
    width: f32,
}

impl<'a> ReplaceView<'a> {
    pub fn new(data: &'a mut ReplaceData, width: f32) -> Self {
        Self { data, width }
    }
}

impl<'a> Widget for ReplaceView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_width(self.width);
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
