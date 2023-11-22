use super::{File, OptionBuilder, Process};
use egui::{Response, Ui, Widget};
use inflector::Inflector;
use std::{fmt::Write, slice::Iter};

/// Select from
/// - `ExtensionOptions::Keep` to not change case (default)
/// - `ExtensionOptions::Lower` to convert to lowercase
/// - `ExtensionOptions::Upper` to convert to uppercase
/// - `ExtensionOptions::Title` to convert to titlecase
/// - `ExtensionOptions::New(&'a str)` to convert to a new extension
/// - `ExtensionOptions::Extra(&'a str)` to add a new extension
/// - `ExtensionOptions::Remove` to remove the extension
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum ExtensionOptions {
    #[default]
    Keep,
    Lower,
    Upper,
    Title,
    New(String),
    Extra(String),
    Remove,
}

impl Process for ExtensionOptions {
    fn process(&self, file: &mut File) {
        match (self, &mut file.extension) {
            (ExtensionOptions::Lower, Some(ext)) => {
                file.extension = Some(ext.to_lowercase());
            }
            (ExtensionOptions::Upper, Some(ext)) => {
                file.extension = Some(ext.to_uppercase());
            }
            (ExtensionOptions::Title, Some(ext)) => {
                file.extension = Some(ext.to_title_case());
            }
            (ExtensionOptions::New(s), _) => {
                file.extension = Some(s.to_string());
            }
            (ExtensionOptions::Extra(s), ext) => {
                match ext {
                    Some(ext) => write!(ext, ".{s}").expect("Unexpected error appending string."),
                    None => file.extension = Some(s.to_string()),
                };
            }
            (ExtensionOptions::Remove, _) => {
                file.extension = None;
            }
            _ => (),
        };
    }
}

impl ExtensionOptions {
    fn iter() -> Iter<'static, ExtensionOptions> {
        static OPTIONS: [ExtensionOptions; 7] = [
            ExtensionOptions::Keep,
            ExtensionOptions::Lower,
            ExtensionOptions::Upper,
            ExtensionOptions::Title,
            ExtensionOptions::New(String::new()),
            ExtensionOptions::Extra(String::new()),
            ExtensionOptions::Remove,
        ];
        OPTIONS.iter()
    }
}

#[derive(Default)]
pub struct ExtensionView {
    options: ExtensionOptions,
    value: String,
    width: f32,
}

impl ExtensionView {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }
}

impl OptionBuilder for ExtensionView {
    type Processor = ExtensionOptions;

    fn build(&self) -> ExtensionOptions {
        match self.options {
            ExtensionOptions::New(_) => ExtensionOptions::New(self.value.clone()),
            ExtensionOptions::Extra(_) => ExtensionOptions::Extra(self.value.clone()),
            _ => self.options.clone(),
        }
    }
}

impl Widget for &mut ExtensionView {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_width(self.width);
            ui.label("Extension");
            ui.horizontal(|ui| {
                egui::ComboBox::new("Extension", "")
                    .selected_text(format!("{:?}", &self.options))
                    .show_ui(ui, |ui| {
                        for opt in ExtensionOptions::iter() {
                            ui.selectable_value(
                                &mut self.options,
                                opt.clone(),
                                format!("{:?}", opt),
                            );
                        }
                    });
                ui.text_edit_singleline(&mut self.value);
            });
        })
        .response
    }
}
#[cfg(test)]
mod extension_tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn test_keep_case() {
        let mut file = File::new(Path::new("test file.txt")).unwrap();
        let opt = ExtensionOptions::Keep;
        opt.process(&mut file);
        assert_eq!(file.extension, Some(String::from("txt")));
    }

    #[test]
    fn test_lower_case() {
        let mut file = File::new(Path::new("test file.TXT")).unwrap();
        let opt = ExtensionOptions::Lower;
        opt.process(&mut file);
        assert_eq!(file.extension, Some(String::from("txt")));
    }

    #[test]
    fn test_upper_case() {
        let mut file = File::new(Path::new("test file.txt")).unwrap();
        let opt = ExtensionOptions::Upper;
        opt.process(&mut file);
        assert_eq!(file.extension, Some(String::from("TXT")));
    }

    #[test]
    fn test_title_case() {
        let mut file = File::new(Path::new("test file.txt")).unwrap();
        let opt = ExtensionOptions::Title;
        opt.process(&mut file);
        assert_eq!(file.extension, Some(String::from("Txt")));
    }

    #[test]
    fn test_new_case() {
        let mut file = File::new(Path::new("test file.txt")).unwrap();
        let opt = ExtensionOptions::New("csv".into());
        opt.process(&mut file);
        assert_eq!(file.extension, Some(String::from("csv")));
    }

    #[test]
    fn test_extra_case_with_existing() {
        let mut file = File::new(Path::new("test file.txt")).unwrap();
        let opt = ExtensionOptions::Extra("bak".into());
        opt.process(&mut file);
        assert_eq!(file.extension, Some(String::from("txt.bak")));
    }

    #[test]
    fn test_extra_case_without_existing() {
        let mut file = File::new(Path::new("test file")).unwrap();
        let opt = ExtensionOptions::Extra("bak".into());
        opt.process(&mut file);
        assert_eq!(file.extension, Some(String::from("bak")));
    }

    #[test]
    fn test_remove() {
        let mut file = File::new(Path::new("test file")).unwrap();
        let opt = ExtensionOptions::Remove;
        opt.process(&mut file);
        assert_eq!(file.extension, None);
    }
}
