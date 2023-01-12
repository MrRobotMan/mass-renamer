use egui::WidgetText;

use super::*;
use valid_text::ValText;

#[derive(Default, Clone)]
pub struct AddData {
    pub prefix: String,
    pub insert: String,
    pub position: i32,
    pub suffix: String,
    pub word_space: bool,
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

#[derive(Default)]
pub struct CaseData {
    pub choice: Case,
    pub snake: bool,
    pub exceptions: String,
}

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

#[derive(Default)]
pub struct ExtensionData {
    pub value: ExtOpts,
    pub new: String,
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum ExtOpts {
    #[default]
    Keep,
    Lower,
    Upper,
    Title,
    New,
    Extra,
    Remove,
}

impl ExtOpts {
    pub fn iterator() -> impl Iterator<Item = ExtOpts> {
        [
            Self::Keep,
            Self::Lower,
            Self::Upper,
            Self::Title,
            Self::New,
            Self::Extra,
            Self::Remove,
        ]
        .iter()
        .copied()
    }
}

#[derive(Debug, Default)]
pub struct FolderData {
    pub position: FolderMode,
    pub sep: String,
    pub levels: ValText<i32>,
}

impl Increment for FolderData {
    fn increment(&mut self, increment: bool, _field: &str) {
        let delta = match increment {
            true => 1,
            false => -1,
        };
        if let Some(val) = self.levels.get_val() {
            self.levels.set_val(val + delta)
        } else if self.levels.is_empty() {
            self.levels.set_val(delta)
        }
    }
}

#[derive(Default)]
pub struct NameData {
    pub value: NameOpts,
    pub new: String,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum NameOpts {
    #[default]
    Keep,
    Remove,
    Fixed,
    Reverse,
}

impl NameOpts {
    pub fn iterator() -> impl Iterator<Item = NameOpts> {
        [Self::Keep, Self::Remove, Self::Fixed, Self::Reverse]
            .iter()
            .copied()
    }
}

#[allow(clippy::from_over_into)]
impl Into<WidgetText> for &NameOpts {
    fn into(self) -> WidgetText {
        WidgetText::RichText(egui::widget_text::RichText::new(match &self {
            NameOpts::Keep => "Keep",
            NameOpts::Remove => "Remove",
            NameOpts::Fixed => "Fixed",
            NameOpts::Reverse => "Reverse",
        }))
    }
}

#[derive(Default)]
pub struct Numberdata {
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

#[derive(Default)]
pub struct RegExData {
    pub exp: String,
    pub replace: String,
    pub extension: bool,
}

pub struct RemoveData {
    pub first_n: ValText<usize>,
    pub last_n: ValText<usize>,
    pub start: ValText<usize>,
    pub end: ValText<usize>,
    pub characters: String,
    pub words: String,
    pub crop_before: bool,
    pub crop: String,
    pub digits: bool,
    pub ascii_high: bool,
    pub trim: bool,
    pub double_space: bool,
    pub chars: bool,
    pub symbols: bool,
    pub lead_dots: bool,
}

impl Increment for RemoveData {
    fn increment(&mut self, increment: bool, field: &str) {
        let delta: i32 = match increment {
            true => 1,
            false => -1,
        };
        let val = match field {
            "first_n" => &mut self.first_n,
            "last_n" => &mut self.last_n,
            "start" => &mut self.start,
            "end" => &mut self.end,
            _ => panic!("Unknown field"),
        };
        if let Some(v) = val.get_val() {
            val.set_val(0.max(v as i32 + delta) as usize)
        } else if val.is_empty() {
            val.set_val(match increment {
                true => 1,
                false => 0,
            })
        };
    }
}

impl Default for RemoveData {
    fn default() -> Self {
        Self {
            first_n: Default::default(),
            last_n: Default::default(),
            start: Default::default(),
            end: Default::default(),
            characters: Default::default(),
            words: Default::default(),
            crop_before: true,
            crop: Default::default(),
            digits: false,
            ascii_high: false,
            trim: false,
            double_space: false,
            chars: false,
            symbols: false,
            lead_dots: false,
        }
    }
}

#[derive(Default)]
pub struct ReplaceData {
    pub replace: String,
    pub with: String,
    pub match_case: bool,
}
