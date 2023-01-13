use crate::{DateFormat, DateMode, DateType};
use egui::{Response, Ui, Widget};

#[derive(Default)]
pub struct DateData<'a> {
    position: DateMode,
    date_type: DateType,
    fmt: DateFormat<'a>,
    sep: String,
    seg: String,
    full_year: bool,
    custom: String,
}

pub struct DateView<'a> {
    data: &'a mut DateData<'a>,
}

impl<'a> DateView<'a> {
    pub fn new(data: &'a mut DateData<'a>) -> Self {
        Self { data }
    }
}

impl<'a> Widget for DateView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        todo!()
    }
}
