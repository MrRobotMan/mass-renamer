use crate::{NumberFormat, NumberMode};
use egui::{Response, Ui, Widget};

#[derive(Default)]
pub struct NumberData {
    choice: NumberMode,
    position: usize,
    start: u32,
    increment: u32,
    pad: usize,
    padding_char: char,
    sep: String,
    reset_pos: Option<usize>,
    format: NumberFormat,
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
