use egui::{Id, Response, Ui, Widget};

use super::{
    increment_decrement::{Arrows, Increment},
    valid_text::ValText,
    NUM_WIDTH,
};

pub struct RemoveData {
    pub first_n: ValText<usize>,
    pub last_n: ValText<usize>,
    pub start: ValText<usize>,
    pub end: ValText<usize>,
    pub characters: String,
    pub words: String,
    pub crop_before: bool,
    pub crop: String,
    pub digits: bool,
    pub ascii_high: bool,
    pub trim: bool,
    pub double_space: bool,
    pub chars: bool,
    pub symbols: bool,
    pub lead_dots: bool,
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
                    .add(
                        egui::TextEdit::singleline(&mut self.data.first_n).desired_width(NUM_WIDTH),
                    )
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
                    .add(egui::TextEdit::singleline(&mut self.data.last_n).desired_width(NUM_WIDTH))
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
                    .add(egui::TextEdit::singleline(&mut self.data.start).desired_width(NUM_WIDTH))
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
                    .add(egui::TextEdit::singleline(&mut self.data.end).desired_width(NUM_WIDTH))
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
        })
        .response
    }
}
