use egui::{self, Id, Sense, Widget};

pub trait Increment {
    fn increment(&mut self, increment: bool) {
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
}

impl<I: Increment> Widget for Arrows<'_, I> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let response = ui.interact(ui.available_rect_before_wrap(), self.id, Sense::hover());
        ui.vertical(|ui| {
            if ui.button("^").clicked() {
                self.value.increment(true)
            };
            if ui.button("v").clicked() {
                self.value.increment(false)
            }
        });
        response
    }
}
