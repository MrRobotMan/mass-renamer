use egui::{ComboBox, Id, Response, TextEdit, Ui, Widget};

use super::{
    increment_decrement::{Arrows, Increment},
    valid_text::ValText,
    NUM_WIDTH,
};

pub struct RemoveData {
    first_n: ValText<usize>,
    last_n: ValText<usize>,
    start: ValText<usize>,
    end: ValText<usize>,
    characters: String,
    words: String,
    crop_before: bool,
    crop: String,
    digits: bool,
    ascii_high: bool,
    trim: bool,
    double_space: bool,
    chars: bool,
    symbols: bool,
    lead_dots: bool,
}

impl Increment for RemoveData {
    fn increment(&mut self, increment: bool, field: &str) {
        let delta: i32 = match increment {
            true => 1,
            false => -1,
        };
        let val = match field {
            "first_n" => &mut self.first_n,
            "last_n" => &mut self.last_n,
            "start" => &mut self.start,
            "end" => &mut self.end,
            _ => panic!("Unknown field"),
        };
        match val.get_val() {
            Some(v) => val.set_val(0.max(v as i32 + delta) as usize),
            None => val.set_val(match increment {
                true => 1,
                false => 0,
            }),
        }
    }
}

impl Default for RemoveData {
    fn default() -> Self {
        Self {
            first_n: Default::default(),
            last_n: Default::default(),
            start: Default::default(),
            end: Default::default(),
            characters: Default::default(),
            words: Default::default(),
            crop_before: true,
            crop: Default::default(),
            digits: false,
            ascii_high: false,
            trim: false,
            double_space: false,
            chars: false,
            symbols: false,
            lead_dots: false,
        }
    }
}

pub struct RemoveView<'a> {
    data: &'a mut RemoveData,
}

impl<'a> RemoveView<'a> {
    pub fn new(data: &'a mut RemoveData) -> Self {
        Self { data }
    }
}

impl<'a> Widget for RemoveView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.label("Remove");
            ui.horizontal(|ui| {
                ui.label("First n");
                if ui
                    .add(TextEdit::singleline(&mut self.data.first_n).desired_width(NUM_WIDTH))
                    .changed()
                    && !self.data.first_n.is_valid()
                {
                    self.data.first_n.revert();
                };
                ui.add(Arrows {
                    id: Id::new("Remove First N"),
                    value: self.data,
                    field: "first_n",
                });
                ui.label("Last n");
                if ui
                    .add(TextEdit::singleline(&mut self.data.last_n).desired_width(NUM_WIDTH))
                    .changed()
                    && !self.data.last_n.is_valid()
                {
                    self.data.last_n.revert();
                };
                ui.add(Arrows {
                    id: Id::new("Remove Last N"),
                    value: self.data,
                    field: "last_n",
                });
            });
            ui.horizontal(|ui| {
                ui.label("Start");
                if ui
                    .add(TextEdit::singleline(&mut self.data.start).desired_width(NUM_WIDTH))
                    .changed()
                    && !self.data.start.is_valid()
                {
                    self.data.start.revert();
                };
                ui.add(Arrows {
                    id: Id::new("Start"),
                    value: self.data,
                    field: "start",
                });
                ui.label("End");
                if ui
                    .add(TextEdit::singleline(&mut self.data.end).desired_width(NUM_WIDTH))
                    .changed()
                    && !self.data.end.is_valid()
                {
                    self.data.end.revert();
                };
                ui.add(Arrows {
                    id: Id::new("End"),
                    value: self.data,
                    field: "end",
                });
            });
            ui.horizontal(|ui| {
                ui.label("Chars");
                ui.text_edit_singleline(&mut self.data.characters);
                ui.label("Words");
                ui.text_edit_singleline(&mut self.data.words);
            });
            ui.horizontal(|ui| {
                ui.label("Crop");
                ComboBox::from_id_source("crop")
                    .selected_text(if self.data.crop_before {
                        "Before"
                    } else {
                        "After"
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.data.crop_before, true, "Before");
                        ui.selectable_value(&mut self.data.crop_before, false, "After");
                    });
                ui.text_edit_singleline(&mut self.data.crop);
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.data.digits, "Digits");
                ui.checkbox(&mut self.data.chars, "Chars");
                ui.checkbox(&mut self.data.ascii_high, "High");
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.data.trim, "Trim");
                ui.checkbox(&mut self.data.double_space, "Double Space");
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.data.lead_dots, "Lead Dots");
                ui.checkbox(&mut self.data.symbols, "Symbols");
            });
        })
        .response
    }
}
