use super::{OptionBuilder, Process, Renamer};
use egui::{Response, Ui, Widget};

/// Options for basic renaming rules.
/// - `replace` - text to be replaced
/// - `with` - new text. Note: the text is always replaced with the text as written, including any specific text case.
/// - `case` - true for case sensitive, false for case-insensitive
#[derive(Default, Debug, Clone)]
pub struct ReplaceOptions {
    pub replace: String,
    pub with: String,
    pub case_sensative: bool,
}

impl Process for ReplaceOptions {
    fn process(&self, file: &mut Renamer) {
        let file = &mut file.stem;
        if self.case_sensative {
            *file = file.replace(&self.replace, &self.with);
        } else {
            while let Some(start) = file.to_lowercase().find(&self.replace.to_lowercase()) {
                let span = self.replace.len();
                for _ in start..(start + span) {
                    file.remove(start);
                }
                file.insert_str(start, &self.with);
            }
        }
    }
}

#[derive(Default)]
pub struct ReplaceView {
    options: ReplaceOptions,
    width: f32,
}

impl ReplaceView {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }
}

impl OptionBuilder for ReplaceView {
    type Processor = ReplaceOptions;
    fn build(&self) -> ReplaceOptions {
        self.options.clone()
    }
}

impl Widget for &mut ReplaceView {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_width(self.width);
            ui.label("Replace");
            ui.horizontal(|ui| {
                ui.label("Replace: ");
                ui.text_edit_singleline(&mut self.options.replace);
            });
            ui.horizontal(|ui| {
                ui.label("With: ");
                ui.text_edit_singleline(&mut self.options.with);
            });
            ui.checkbox(&mut self.options.case_sensative, "Match Case")
        })
        .response
    }
}

#[cfg(test)]
mod match_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn no_matching_text_case_sensitive() {
        let replace = "ABC".into();
        let with = "123".into();
        let mut file = Renamer::new(Path::new("fileabc")).unwrap();
        let case_sensative = true;
        let opt = ReplaceOptions {
            replace,
            with,
            case_sensative,
        };
        opt.process(&mut file);
        assert_eq!(file.stem, String::from("fileabc"))
    }

    #[test]
    fn no_matching_text_case_insensitive() {
        let replace = "qrs".into();
        let with = "123".into();
        let mut file = Renamer::new(Path::new("fileabc")).unwrap();
        let case = false;
        let opt = ReplaceOptions {
            replace,
            with,
            case_sensative: case,
        };
        opt.process(&mut file);
        assert_eq!(file.stem, String::from("fileabc"))
    }

    #[test]
    fn matched_case_sensitive() {
        let replace = "abc".into();
        let with = "123".into();
        let mut file = Renamer::new(Path::new("fileabc")).unwrap();
        let case = true;
        let opt = ReplaceOptions {
            replace,
            with,
            case_sensative: case,
        };
        opt.process(&mut file);
        assert_eq!(file.stem, String::from("file123"))
    }

    #[test]
    fn matched_case_insensitive() {
        let replace = "ABC".into();
        let with = "123".into();
        let mut file = Renamer::new(Path::new("fileabc")).unwrap();
        let case = false;
        let opt = ReplaceOptions {
            replace,
            with,
            case_sensative: case,
        };
        opt.process(&mut file);
        assert_eq!(file.stem, String::from("file123"))
    }

    #[test]
    fn test_multiple() {
        let replace = "-".into();
        let with = "_".into();
        let case_sensative = false;
        let mut file = Renamer::new(Path::new("12-34-56")).unwrap();
        let opt = ReplaceOptions {
            replace,
            with,
            case_sensative,
        };
        opt.process(&mut file);
        assert_eq!(file.stem, String::from("12_34_56"))
    }
}
