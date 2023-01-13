use crate::{DateFormat, DateMode, DateType};
use egui::{Response, Ui, Widget};

#[derive(Default)]
pub struct DateData<'a> {
    pub position: DateMode,
    pub date_type: DateType,
    pub fmt: DateFormat<'a>,
    pub sep: String,
    pub seg: String,
    pub full_year: bool,
    pub custom: String,
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
