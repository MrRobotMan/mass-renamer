use crate::case::Case;
use egui::{Response, Ui, Widget};

#[derive(Default)]
pub struct CaseData {
    choice: Case,
    snake: bool,
    exceptions: String,
}

pub struct CaseView<'a> {
    data: &'a mut CaseData,
}

impl<'a> CaseView<'a> {
    pub fn new(data: &'a mut CaseData) -> Self {
        Self { data }
    }
}

impl<'a> Widget for CaseView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.label("Case");
            ui.horizontal(|ui| {
                egui::ComboBox::new("Case", "")
                    .selected_text(format!("{:?}", &self.data.choice))
                    .show_ui(ui, |ui| {
                        for opt in Case::iterator() {
                            ui.selectable_value(&mut self.data.choice, opt, format!("{:?}", opt));
                        }
                    });
                ui.checkbox(&mut self.data.snake, "Snake_Case")
            });
            ui.horizontal(|ui| {
                ui.label("Except:");
                ui.text_edit_singleline(&mut self.data.exceptions);
            });
        })
        .response
    }
}
