use crate::{NumberFormat, NumberMode};
use egui::{Response, Ui, Widget};

#[derive(Default)]
pub struct NumberData {
    pub choice: NumberMode,
    pub position: usize,
    pub start: u32,
    pub increment: u32,
    pub pad: usize,
    pub padding_char: char,
    pub sep: String,
    pub reset_pos: Option<usize>,
    pub format: NumberFormat,
}

pub struct NumberView<'a> {
    data: &'a mut NumberData,
}

impl<'a> NumberView<'a> {
    pub fn new(data: &'a mut NumberData) -> Self {
        Self { data }
    }
}

impl<'a> Widget for NumberView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        todo!()
    }
}
