use super::{
    increment_decrement::{Arrows, Increment},
    valid_text::ValText,
};
use crate::AddOptions;
use egui::{Response, Ui, Widget};

#[derive(Default)]
pub struct AddData {
    prefix: String,
    insert: String,
    position: ValText<i32>,
    suffix: String,
    word_space: bool,
}

impl AddData {
    pub fn _make_options(&self) -> AddOptions {
        let prefix = match &self.prefix {
            s if s.is_empty() => None,
            s => Some(s.as_str()),
        };
        let insert = match &self.insert {
            s if s.is_empty() => None,
            s => Some((self.position.get_val().unwrap_or(0), s.as_str())),
        };
        let suffix = match &self.suffix {
            x if x.is_empty() => None,
            s => Some(s.as_str()),
        };
        AddOptions {
            prefix,
            insert,
            suffix,
            word_space: self.word_space,
        }
    }
}

impl Increment for AddData {
    fn increment(&mut self, increment: bool, _field: &str) {
        let delta = match increment {
            true => 1,
            false => -1,
        };
        match self.position.get_val() {
            Some(v) => self.position.set_val(v + delta),
            None => self.position.set_val(delta),
        };
    }
}

pub struct AddView<'a> {
    data: &'a mut AddData,
    width: f32,
}

impl<'a> AddView<'a> {
    pub fn new(data: &'a mut AddData, width: f32) -> Self {
        Self { data, width }
    }
}

impl<'a> Widget for AddView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_width(self.width);
            ui.label("Add");
            ui.horizontal(|ui| {
                ui.label("Prefix");
                ui.text_edit_singleline(&mut self.data.prefix);
            });
            ui.horizontal(|ui| {
                ui.label("Insert");
                ui.text_edit_singleline(&mut self.data.insert);
            });
            ui.horizontal(|ui| {
                ui.label("at:");
                if ui.text_edit_singleline(&mut self.data.position).changed()
                    && !self.data.position.is_valid()
                {
                    self.data.position.revert();
                };
                ui.add(Arrows::new("position", self.data, ""))
            });
            ui.horizontal(|ui| {
                ui.label("Suffix");
                ui.text_edit_singleline(&mut self.data.suffix);
            });
            ui.checkbox(&mut self.data.word_space, "Word Space");
        })
        .response
    }
}
