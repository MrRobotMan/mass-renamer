use super::{
    increment_decrement::{Arrows, Increment},
    valid_text::ValText,
};
use crate::{NumberFormat, NumberMode};
use egui::{ComboBox, Response, Ui, Widget};

#[derive(Default)]
pub struct NumberData {
    choice: NumberMode,
    position: ValText<usize>,
    start: ValText<usize>,
    increment: ValText<usize>,
    pad: ValText<usize>,
    padding_char: ValText<char>,
    sep: String,
    reset_pos: ValText<usize>,
    format: NumberFormat,
}

impl Increment for NumberData {
    fn increment(&mut self, increment: bool, field: &str) {
        let delta = match increment {
            true => 1,
            false => -1,
        };
        let current = match field {
            "position" => &mut self.position,
            "start" => &mut self.start,
            "increment" => &mut self.increment,
            "pad" => &mut self.pad,
            "reset_pos" => &mut self.reset_pos,
            _ => panic!("Unexpected field match in NumberData. Found {field}"),
        };
        if let Some(v) = current.get_val() {
            current.set_val(0.max(v as i32 + delta) as usize);
        };
    }
}

pub struct NumberView<'a> {
    data: &'a mut NumberData,
    width: f32,
}

impl<'a> NumberView<'a> {
    pub fn new(data: &'a mut NumberData, width: f32) -> Self {
        Self { data, width }
    }
}

impl<'a> Widget for NumberView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_width(self.width);
            ui.label("Numbering");
            ui.horizontal(|ui| {
                ui.set_width(self.width);
                ui.label("Mode");
                let response = ComboBox::from_id_source("Number Mode")
                    .selected_text(match self.data.choice {
                        NumberMode::Prefix => "Prefix",
                        NumberMode::Suffix => "Suffix",
                        NumberMode::Insert(_) => "Insert",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.data.choice, NumberMode::Prefix, "Prefix");
                        ui.selectable_value(&mut self.data.choice, NumberMode::Suffix, "Suffix");
                        ui.selectable_value(&mut self.data.choice, NumberMode::Insert(0), "Insert");
                    })
                    .response;
                if response.changed() {
                    match &self.data.choice {
                        NumberMode::Insert(_) => {
                            self.data.choice =
                                NumberMode::Insert(self.data.position.get_val().unwrap_or(0));
                        }
                        _ => self.data.position.set_val(0),
                    }
                };
                ui.label("at:");
                if ui.text_edit_singleline(&mut self.data.position).changed() {
                    if !self.data.position.is_valid() {
                        self.data.position.revert()
                    } else {
                        self.data.choice =
                            NumberMode::Insert(self.data.position.get_val().unwrap());
                    };
                }
            });
            ui.horizontal(|ui| {
                ui.set_width(self.width);
                ui.label("Start");
                if ui.text_edit_singleline(&mut self.data.start).changed()
                    && !self.data.start.is_valid()
                {
                    self.data.start.revert();
                }
                ui.add(Arrows::new("Number Start", self.data, "start"));
                ui.label("Incr.");
                if ui.text_edit_singleline(&mut self.data.increment).changed()
                    && !self.data.increment.is_valid()
                {
                    self.data.increment.revert();
                }
                ui.add(Arrows::new("Number Increment", self.data, "increment"));
            });
            ui.horizontal(|ui| {
                ui.set_width(self.width);
                ui.label("Pad");
                if ui.text_edit_singleline(&mut self.data.pad).changed()
                    && !self.data.pad.is_valid()
                {
                    self.data.pad.revert();
                }
                ui.add(Arrows::new("Number Pad", self.data, "pad"));
                ui.label("Char");
                if ui
                    .text_edit_singleline(&mut self.data.padding_char)
                    .changed()
                    && !self.data.padding_char.is_valid()
                {
                    if let Some(c) = self.data.padding_char.get_text().chars().last() {
                        self.data.padding_char.set_val(c)
                    };
                };
            });
            ui.horizontal(|ui| {
                ui.set_width(self.width);
                ui.label("Reset Val.");
                if ui.text_edit_singleline(&mut self.data.reset_pos).changed()
                    && !self.data.reset_pos.is_valid()
                {
                    self.data.reset_pos.revert();
                }
                ui.add(Arrows::new("Number Reset", self.data, "reset_pos"));
            });
            ui.horizontal(|ui| {
                ui.set_width(self.width);
                ui.label("Format");
                ComboBox::from_id_source("Number Format")
                    .selected_text(match self.data.format {
                        NumberFormat::Binary => "Binary",
                        NumberFormat::Decimal => "Decimal",
                        NumberFormat::HexUpper => "Hex Upper",
                        NumberFormat::HexLower => "Hex Lower",
                        NumberFormat::Octal => "Octal",
                        NumberFormat::AsciiUpper => "A-Z",
                        NumberFormat::AsciiLower => "a-z",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.data.format, NumberFormat::Binary, "Binary");
                        ui.selectable_value(&mut self.data.format, NumberFormat::Octal, "Octal");
                        ui.selectable_value(
                            &mut self.data.format,
                            NumberFormat::Decimal,
                            "Decimal",
                        );
                        ui.selectable_value(
                            &mut self.data.format,
                            NumberFormat::HexUpper,
                            "Hex Upper",
                        );
                        ui.selectable_value(
                            &mut self.data.format,
                            NumberFormat::HexLower,
                            "Hex Lower",
                        );
                        ui.selectable_value(&mut self.data.format, NumberFormat::AsciiUpper, "A-Z");
                        ui.selectable_value(&mut self.data.format, NumberFormat::AsciiLower, "a-z");
                    });
            });
        })
        .response
    }
}
