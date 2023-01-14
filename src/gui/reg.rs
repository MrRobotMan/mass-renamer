use egui::{Response, Ui, Widget};

#[derive(Default)]
pub struct RegExData {
    exp: String,
    replace: String,
    extension: bool,
}

pub struct RegExView<'a> {
    data: &'a mut RegExData,
    width: f32,
}

impl<'a> RegExView<'a> {
    pub fn new(data: &'a mut RegExData, width: f32) -> Self {
        Self { data, width }
    }
}

impl<'a> Widget for RegExView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_width(self.width);
            ui.label("Regex");
            ui.horizontal(|ui| {
                ui.label("Match:");
                ui.text_edit_singleline(&mut self.data.exp);
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
