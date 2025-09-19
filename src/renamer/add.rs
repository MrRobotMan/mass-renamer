use egui::{Response, Ui, Widget};

use super::{OptionBuilder, Process, Renamer};
// use crate::gui::{Arrows, Incrementer, ValText};

/// Add a fixed `Prefix` or`Suffix` to the filename,
/// or `Insert` text at a specific location (0 indexed, negative to index from the end).
///
/// You may also choose to add a `Word Space`. This will insert a space before any
/// capital letter (except the first character), unless there's a space already there.
#[derive(Default, Debug, Clone)]
pub struct AddOptions {
    pub prefix: Option<String>,
    pub insert: Option<(i32, String)>,
    pub suffix: Option<String>,
    pub word_space: bool,
}

impl Process for AddOptions {
    fn process(&self, file: &mut Renamer) {
        let file = &mut file.stem;
        if let Some(prefix) = &self.prefix {
            file.insert_str(0, prefix);
        }

        if let Some((pos, insert)) = &self.insert {
            match pos {
                p if p >= &(file.len() as i32) => file.push_str(insert),
                p if p >= &0 => file.insert_str(*p as usize, insert),
                p if -p >= file.len() as i32 => file.insert_str(0, insert),
                _ => {
                    let p = (file.len() as i32 + pos) as usize;
                    file.insert_str(p, insert);
                } // pos is negative
            }
        }

        if let Some(suffix) = &self.suffix {
            file.push_str(suffix);
        }

        if self.word_space {
            let mut new = String::new();
            let mut iter = file.chars();
            if let Some(chr) = iter.next() {
                new.push(chr)
            }
            for chr in iter {
                if chr.is_uppercase() {
                    new.push(' ');
                }
                new.push(chr);
            }
            *file = new
        }
    }
}

/*
#[derive(Default)]
pub struct AddView {
    prefix: String,
    insert: String,
    position: ValText<i32>,
    suffix: String,
    word_space: bool,
    width: f32,
}

impl AddView {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }
}

impl OptionBuilder for AddView {
    type Processor = AddOptions;
    fn build(&self) -> AddOptions {
        let prefix = match &self.prefix {
            s if s.is_empty() => None,
            s => Some(s.clone()),
        };
        let insert = match &self.insert {
            s if s.is_empty() => None,
            s => Some((self.position.get_val().unwrap_or(0), s.clone())),
        };
        let suffix = match &self.suffix {
            x if x.is_empty() => None,
            s => Some(s.clone()),
        };
        AddOptions {
            prefix,
            insert,
            suffix,
            word_space: self.word_space,
        }
    }
}

impl Incrementer for &mut AddView {
    fn increment(&mut self, _field: &str) {
        match self.position.get_val() {
            Some(v) => self.position.set_val(v + 1),
            None => self.position.set_val(1),
        };
    }
    fn decrement(&mut self, _field: &str) {
        match self.position.get_val() {
            Some(v) => self.position.set_val(v + -1),
            None => self.position.set_val(-1),
        };
    }
}
impl Widget for &mut AddView {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_width(self.width);
            ui.label("Add");
            ui.horizontal(|ui| {
                ui.label("Prefix");
                ui.text_edit_singleline(&mut self.prefix);
            });
            ui.horizontal(|ui| {
                ui.label("Insert");
                ui.text_edit_singleline(&mut self.insert);
            });
            ui.horizontal(|ui| {
                ui.label("at:");
                if ui.text_edit_singleline(&mut self.position).changed()
                    && !self.position.is_valid()
                {
                    self.position.revert();
                };
            });
            ui.add(Arrows::new("position", &mut self, ""));
            ui.horizontal(|ui| {
                ui.label("Suffix");
                ui.text_edit_singleline(&mut self.suffix);
            });
            ui.checkbox(&mut self.word_space, "Word Space");
        })
        .response
    }
}
*/

#[cfg(test)]
mod add_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn add_all_options() {
        let prefix = Some("prefix-".into());
        let insert = Some((15, "-insert-".into()));
        let suffix = Some("-suffix".into());
        let word_space = true;
        let file = Path::new("SomeTestFile");
        let opt = AddOptions {
            prefix,
            insert,
            suffix,
            word_space,
        };
        let mut rename = Renamer::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(
            rename.stem,
            "prefix- Some Test-insert- File-suffix".to_owned()
        )
    }

    #[test]
    fn test_negative_insert() {
        let insert = Some((-1, "!".into()));
        let file = Path::new("Some Test File");
        let opt = AddOptions {
            prefix: None,
            insert,
            suffix: None,
            word_space: false,
        };
        let mut rename = Renamer::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(rename.stem, "Some Test Fil!e".to_owned());
    }

    #[test]
    fn test_insert_too_far_positive() {
        let insert = Some((100, "!".into()));
        let file = Path::new("Some Test File");
        let opt = AddOptions {
            prefix: None,
            insert,
            suffix: None,
            word_space: false,
        };
        let mut rename = Renamer::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(rename.stem, "Some Test File!".to_owned());
    }

    #[test]
    fn test_insert_too_far_negative() {
        let insert = Some((-100, "!".into()));
        let file = Path::new("Some Test File");
        let opt = AddOptions {
            prefix: None,
            insert,
            suffix: None,
            word_space: false,
        };
        let mut rename = Renamer::new(file).unwrap();
        opt.process(&mut rename);
        assert_eq!(rename.stem, "!Some Test File".to_owned());
    }
}
