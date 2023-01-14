use crate::{DateMode, DatePrefix, DateSuffix, DateType};
use egui::{Checkbox, ComboBox, Response, Ui, Widget};

#[derive(Default)]
pub struct DateData {
    position: Option<DateMode>,
    date_type: DateType,
    fmt: Format,
    custom: String,
    sep: String,
    seg: String,
    full_year: bool,
}

#[derive(Default, PartialEq, Copy, Clone)]
enum Format {
    #[default]
    DayMoYr,
    MoDayYr,
    YrMoDay,
    DayMoYrHrMin,
    MoDayYrHrMin,
    YrMmDayHrMin,
    DayMoYrHrMinSec,
    MoDayYrHrMinSec,
    YrMoDayHrMinSec,
    Custom,
}

impl Format {
    fn value(&self) -> &str {
        match self {
            Self::DayMoYr => "DMY",
            Self::MoDayYr => "MDY",
            Self::YrMoDay => "YMD",
            Self::DayMoYrHrMin => "DMY HM",
            Self::MoDayYrHrMin => "MDY HM",
            Self::YrMmDayHrMin => "YMD HM",
            Self::DayMoYrHrMinSec => "DMY HMS",
            Self::MoDayYrHrMinSec => "MDY HMS",
            Self::YrMoDayHrMinSec => "YMD HMS",
            Self::Custom => "Custom",
        }
    }

    fn iter() -> impl Iterator<Item = Format> {
        [
            Self::DayMoYr,
            Self::MoDayYr,
            Self::YrMoDay,
            Self::DayMoYrHrMin,
            Self::MoDayYrHrMin,
            Self::YrMmDayHrMin,
            Self::DayMoYrHrMinSec,
            Self::MoDayYrHrMinSec,
            Self::YrMoDayHrMinSec,
            Self::Custom,
        ]
        .iter()
        .copied()
    }
}

pub struct DateView<'a> {
    data: &'a mut DateData,
}

impl<'a> DateView<'a> {
    pub fn new(data: &'a mut DateData) -> Self {
        Self { data }
    }
}

impl<'a> Widget for DateView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.label("Date");
            ui.horizontal(|ui| {
                ui.label("Mode");
                ComboBox::from_id_source("Date Mode")
                    .selected_text(match self.data.position {
                        Some(DateMode::Prefix) => "Prefix",
                        Some(DateMode::Suffix) => "Suffix",
                        None => "None",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.data.position, None, "None");
                        ui.selectable_value(
                            &mut self.data.position,
                            Some(DateMode::Prefix),
                            "Prefix",
                        );
                        ui.selectable_value(
                            &mut self.data.position,
                            Some(DateMode::Suffix),
                            "Suffix",
                        );
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Type");
                ComboBox::from_id_source("Date Type")
                    .selected_text(match self.data.date_type {
                        DateType::Created => "Created",
                        DateType::Modified => "Modified",
                        DateType::Current => "Now",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.data.date_type, DateType::Created, "Created");
                        ui.selectable_value(
                            &mut self.data.date_type,
                            DateType::Modified,
                            "Modified",
                        );
                        ui.selectable_value(&mut self.data.date_type, DateType::Current, "Now");
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Format");
                if ComboBox::from_id_source("Date Fmt")
                    .selected_text(self.data.fmt.value())
                    .show_ui(ui, |ui| {
                        for opt in Format::iter() {
                            ui.selectable_value(&mut self.data.fmt, opt, opt.value());
                        }
                    })
                    .response
                    .changed()
                    && self.data.fmt != Format::Custom
                {
                    self.data.custom = String::new();
                };
            });

            ui.horizontal(|ui| {
                ui.label("Custom");
                if ui.text_edit_singleline(&mut self.data.custom).changed()
                    && !self.data.custom.is_empty()
                {
                    self.data.fmt = Format::Custom;
                };
            });
            ui.horizontal(|ui| {
                ui.label("Sep.");
                ui.text_edit_singleline(&mut self.data.sep);
                ui.label("Seg");
                ui.text_edit_singleline(&mut self.data.seg);
            });
            ui.checkbox(&mut self.data.full_year, "4 Digit Year");
        })
        .response
    }
}
