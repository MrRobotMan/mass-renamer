use crate::AddOptions;
use egui::{Response, Ui, Widget};

#[derive(Default, Clone)]
pub struct AddData {
    prefix: String,
    insert: String,
    position: i32,
    suffix: String,
    word_space: bool,
}

impl AddData {
    pub fn _make_options(&self) -> AddOptions {
        let prefix = match &self.prefix {
            s if s.is_empty() => None,
            s => Some(s.as_str()),
        };
        let insert = match &self.insert {
            s if s.is_empty() => None,
            s => Some((self.position, s.as_str())),
        };
        let suffix = match &self.suffix {
            x if x.is_empty() => None,
            s => Some(s.as_str()),
        };
        AddOptions {
            prefix,
            insert,
            suffix,
            word_space: self.word_space,
        }
    }
}

pub struct AddView<'a> {
    data: &'a mut AddData,
}

impl<'a> AddView<'a> {
    pub fn new(data: &'a mut AddData) -> Self {
        Self { data }
    }
}

impl<'a> Widget for AddView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        todo!()
    }
}
