use super::{File, OptionBuilder, Process};
use regex::Regex;

use egui::{Response, Ui, Widget};

/// Use a regular expression `exp` to find the offending text and `rep` it with new.
///
/// Using the `extension` boolean to declare whether to search the file extension too.
#[derive(Default, Debug, Clone)]
pub struct RegexOptions {
    pub exp: String,
    pub rep: String,
    pub extension: bool,
}

impl Process for RegexOptions {
    fn process(&self, file: &mut File) {
        if let Ok(exp) = Regex::new(&self.exp) {
            match (self.extension, &file.extension) {
                (false, _) | (_, None) => {
                    file.stem = exp.replace_all(&file.stem, &self.rep).to_string()
                }
                (true, Some(ext)) => {
                    let mut f = file.stem.clone();
                    f.push('.');
                    f.push_str(ext);
                    let res = exp.replace_all(&f, &self.rep).to_string();
                    match res.rsplit_once('.') {
                        None => file.stem = res,
                        Some((s, e)) => {
                            file.stem = s.to_owned();
                            file.extension = Some(e.to_owned());
                        }
                    }
                }
            }
        };
    }
}

#[derive(Default)]
pub struct RegexView {
    options: RegexOptions,
    width: f32,
}

impl RegexView {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }
}

impl OptionBuilder for RegexView {
    type Processor = RegexOptions;

    fn build(&self) -> RegexOptions {
        self.options.clone()
    }
}

impl Widget for &mut RegexView {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_width(self.width);
            ui.label("Regex");
            ui.horizontal(|ui| {
                ui.label("Match:");
                ui.text_edit_singleline(&mut self.options.exp);
            });
            ui.horizontal(|ui| {
                ui.label("Replacement:");
                ui.text_edit_singleline(&mut self.options.rep)
            });
            ui.checkbox(&mut self.options.extension, "Include Extension");
        })
        .response
    }
}

#[cfg(test)]
mod regex_tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn regex_test_with_extension() {
        let exp = "0123.txt".into();
        let file = Path::new("./file0123.txt");
        let rep = "ABCD.csv".into();
        let opt = RegexOptions {
            exp,
            rep,
            extension: true,
        };
        let mut rename = File::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(
            (rename.stem, rename.extension),
            (String::from("fileABCD"), Some(String::from("csv")))
        );
    }
    #[test]
    fn regex_test_no_extension() {
        let exp = "0123".into();
        let rep = "ABCD".into();
        let file = Path::new("./file0123.txt");
        let opt = RegexOptions {
            exp,
            rep,
            extension: false,
        };
        let mut rename = File::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(
            (rename.stem, rename.extension),
            (String::from("fileABCD"), Some(String::from("txt")))
        );
    }
    #[test]
    fn regex_test_no_extension_no_match() {
        let exp = "0123".into();
        let rep = "ABCD".into();
        let file = Path::new("./file123.txt");
        let opt = RegexOptions {
            exp,
            rep,
            extension: false,
        };
        let mut rename = File::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(
            (rename.stem, rename.extension),
            (String::from("file123"), Some(String::from("txt")))
        );
    }
}
