use egui::{Response, Ui, Widget, WidgetText};

#[derive(Default)]
pub struct NameData {
    pub value: NameOpts,
    pub new: String,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum NameOpts {
    #[default]
    Keep,
    Remove,
    Fixed,
    Reverse,
}

impl NameOpts {
    pub fn iterator() -> impl Iterator<Item = NameOpts> {
        [Self::Keep, Self::Remove, Self::Fixed, Self::Reverse]
            .iter()
            .copied()
    }
}

#[allow(clippy::from_over_into)]
impl Into<WidgetText> for &NameOpts {
    fn into(self) -> WidgetText {
        WidgetText::RichText(egui::widget_text::RichText::new(match &self {
            NameOpts::Keep => "Keep",
            NameOpts::Remove => "Remove",
            NameOpts::Fixed => "Fixed",
            NameOpts::Reverse => "Reverse",
        }))
    }
}

pub struct NameView<'a> {
    data: &'a mut NameData,
}

impl<'a> NameView<'a> {
    pub fn new(data: &'a mut NameData) -> Self {
        Self { data }
    }
}

impl<'a> Widget for NameView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.label("Name");
            egui::ComboBox::new("Name Options", "")
                .selected_text(&self.data.value)
                .show_ui(ui, |ui| {
                    for opt in NameOpts::iterator() {
                        ui.selectable_value(&mut self.data.value, opt, format!("{:?}", opt));
                    }
                });
            ui.text_edit_singleline(&mut self.data.new);
        })
        .response
    }
}
