use std::slice::Iter;

use super::{OptionBuilder, Process, Renamer};
use egui::{Response, RichText, Ui, Widget, WidgetText};

/// Select from.
/// - `NameOptions::Keep` - Do not change the original file name (default).
/// - `NameOptions::Remove` - Completely erase the file from the selected items. This allows it to be rebuilt using components higher than (2).
/// - `NameOptions::Fixed` - Specify a new file in the box for all selected items. Only really useful if you're also using the Numbering section.
/// - `NameOptions::Reverse` - Reverse the name, e.g. 12345.txt becomes 54321.txt.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum NameOptions {
    #[default]
    Keep,
    Remove,
    Fixed(String),
    Reverse,
}

impl Process for NameOptions {
    fn process(&self, file: &mut Renamer) {
        match self {
            NameOptions::Keep => (),
            NameOptions::Remove => file.stem = "".to_owned(),
            NameOptions::Fixed(x) => file.stem = x.to_string(),
            NameOptions::Reverse => file.stem = file.stem.chars().rev().collect::<String>(),
        };
    }
}

impl NameOptions {
    fn iter() -> Iter<'static, Self> {
        static OPTIONS: [NameOptions; 4] = [
            NameOptions::Keep,
            NameOptions::Remove,
            NameOptions::Fixed(String::new()),
            NameOptions::Reverse,
        ];
        OPTIONS.iter()
    }
}

impl From<&NameOptions> for WidgetText {
    fn from(value: &NameOptions) -> Self {
        WidgetText::RichText(RichText::new(match value {
            NameOptions::Keep => "Keep",
            NameOptions::Remove => "Remove",
            NameOptions::Fixed(_) => "Fixed",
            NameOptions::Reverse => "Reverse",
        }))
    }
}

#[derive(Default)]
pub struct NameView {
    mode: NameOptions,
    value: String,
    width: f32,
}

impl NameView {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }
}

impl OptionBuilder for NameView {
    type Processor = NameOptions;

    fn build(&self) -> NameOptions {
        match self.mode {
            NameOptions::Fixed(_) => NameOptions::Fixed(self.value.clone()),
            _ => self.mode.clone(),
        }
    }
}

impl Widget for &mut NameView {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_width(self.width);
            ui.label("Name");
            egui::ComboBox::new("Name Options", "")
                .selected_text(&self.mode)
                .show_ui(ui, |ui| {
                    for opt in NameOptions::iter() {
                        ui.selectable_value(&mut self.mode, opt.clone(), format!("{:?}", opt));
                    }
                });
            ui.text_edit_singleline(&mut self.value);
        })
        .response
    }
}
#[cfg(test)]
mod name_tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn keep_name() {
        let mut file = Renamer::new(Path::new("file")).unwrap();
        let opt = NameOptions::Keep;
        opt.process(&mut file);
        assert_eq!(&file.stem, "file");
    }
    #[test]
    fn remove_name() {
        let mut file = Renamer::new(Path::new("file")).unwrap();
        let opt = NameOptions::Remove;
        opt.process(&mut file);
        assert_eq!(&file.stem, "");
    }
    #[test]
    fn fixed_name() {
        let mut file = Renamer::new(Path::new("file")).unwrap();
        let new_name = "renamed_file";
        let opt = NameOptions::Fixed(String::from(new_name));
        opt.process(&mut file);
        assert_eq!(file.stem, new_name);
    }
    #[test]
    fn reverse_name() {
        let mut file = Renamer::new(Path::new("file")).unwrap();
        let opt = NameOptions::Reverse;
        opt.process(&mut file);
        assert_eq!(&file.stem, "elif");
    }
}
