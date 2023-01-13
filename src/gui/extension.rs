use egui::{Response, Ui, Widget};

#[derive(Default)]
pub struct ExtensionData {
    value: ExtOpts,
    new: String,
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum ExtOpts {
    #[default]
    Keep,
    Lower,
    Upper,
    Title,
    New,
    Extra,
    Remove,
}

impl ExtOpts {
    pub fn iterator() -> impl Iterator<Item = ExtOpts> {
        [
            Self::Keep,
            Self::Lower,
            Self::Upper,
            Self::Title,
            Self::New,
            Self::Extra,
            Self::Remove,
        ]
        .iter()
        .copied()
    }
}

pub struct ExtensionView<'a> {
    data: &'a mut ExtensionData,
}

impl<'a> ExtensionView<'a> {
    pub fn new(data: &'a mut ExtensionData) -> Self {
        Self { data }
    }
}

impl<'a> Widget for ExtensionView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.label("Extension");
            ui.horizontal(|ui| {
                egui::ComboBox::new("Extension", "")
                    .selected_text(format!("{:?}", &self.data.value))
                    .show_ui(ui, |ui| {
                        for opt in ExtOpts::iterator() {
                            ui.selectable_value(&mut self.data.value, opt, format!("{:?}", opt));
                        }
                    });
                ui.text_edit_singleline(&mut self.data.new);
            });
        })
        .response
    }
}
