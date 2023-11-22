use egui::{self, Id, Response, Ui, Widget};

pub trait Incrementer {
    fn increment(&mut self, field: &str);
    fn decrement(&mut self, field: &str);
}

/// A set of increment decrement arrows stacked vertically.
pub struct Arrows<'a, I: Incrementer> {
    pub id: Id,
    pub value: &'a mut I,
    pub field: &'a str,
}

impl<'a, I: Incrementer> Arrows<'a, I> {
    pub fn new(id: &str, value: &'a mut I, field: &'a str) -> Self {
        Self {
            id: Id::new(id),
            value,
            field,
        }
    }
}

impl<I: Incrementer> Widget for Arrows<'_, I> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_width(10.0);
            if ui.button("^").clicked() {
                self.value.increment(self.field)
            };
            if ui.button("v").clicked() {
                self.value.decrement(self.field)
            }
        })
        .response
    }
}
