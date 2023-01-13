use egui::{Response, Ui, Widget};

#[derive(Default)]
pub struct RegExData {
    pub exp: String,
    pub replace: String,
    pub extension: bool,
}

pub struct RegExView<'a> {
    data: &'a mut RegExData,
}

impl<'a> RegExView<'a> {
    pub fn new(data: &'a mut RegExData) -> Self {
        Self { data }
    }
}

impl<'a> Widget for RegExView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.label("Regex");
            ui.horizontal(|ui| {
                ui.label("Match:");
                ui.text_edit_singleline(&mut self.data.exp)
            });
            ui.horizontal(|ui| {
                ui.label("Replacement:");
                ui.text_edit_singleline(&mut self.data.replace)
            });
            ui.checkbox(&mut self.data.extension, "Include Extension");
        })
        .response
    }
}
