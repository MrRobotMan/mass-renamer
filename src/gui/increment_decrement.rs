use egui::{self, Id, Response, Ui, Widget};

pub trait Increment {
    fn increment(&mut self, increment: bool, field: &str);
}

/// A set of increment decrement arrows stacked vertically.
pub struct Arrows<'a, I: Increment> {
    pub id: Id,
    pub value: &'a mut I,
    pub field: &'a str,
}

impl<I: Increment> Widget for Arrows<'_, I> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            if ui.button("^").clicked() {
                self.value.increment(true, self.field)
            };
            if ui.button("v").clicked() {
                self.value.increment(false, self.field)
            }
        })
        .response
    }
}
