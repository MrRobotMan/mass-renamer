use egui::{self, Id, Response, Sense, Ui, Widget};

pub trait Increment {
    fn increment(&mut self, increment: bool, _field: &str) {
        let _delta = match increment {
            true => 1,
            false => -1,
        };
    }
}

/// A set of increment decrement arrows stacked vertically.
pub struct Arrows<'t, I: Increment> {
    pub id: Id,
    pub value: &'t mut I,
    pub field: &'t str,
}

impl<I: Increment> Widget for Arrows<'_, I> {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.interact(ui.available_rect_before_wrap(), self.id, Sense::hover());
        ui.vertical(|ui| {
            if ui.button("^").clicked() {
                self.value.increment(true, self.field)
            };
            if ui.button("v").clicked() {
                self.value.increment(false, self.field)
            }
        });
        response
    }
}
