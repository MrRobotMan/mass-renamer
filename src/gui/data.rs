use crate::file;

#[derive(Default)]
pub struct AddData {
    pub prefix: String,
    pub insert: String,
    pub position: i32,
    pub suffix: String,
    pub word_space: bool,
}

#[derive(Default)]
pub struct CaseData {
    pub choice: file::Case,
    pub snake: bool,
    pub exceptions: String,
}

#[derive(Default)]
pub struct DateData<'a> {
    pub position: file::DateMode,
    pub date_type: file::DateType,
    pub fmt: file::DateFormat<'a>,
    pub sep: String,
    pub seg: String,
    pub full_year: bool,
    pub custom: String,
}

#[derive(Default)]
pub struct ExtensionData<'a> {
    pub value: file::ExtensionOptions<'a>,
    pub new: String,
}

#[derive(Default)]
pub struct Folderdata {
    pub postion: file::FolderMode,
    pub sep: String,
    pub levels: i32,
}

#[derive(Default)]
pub struct NameData<'a> {
    pub value: file::NameOptions<'a>,
    pub new: String,
}

#[derive(Default)]
pub struct Numberdata {
    pub choice: file::NumberMode,
    pub position: usize,
    pub start: u32,
    pub increment: u32,
    pub pad: usize,
    pub padding_char: char,
    pub sep: String,
    pub reset_pos: Option<usize>,
    pub format: file::NumberFormat,
}

#[derive(Default)]
pub struct RegExData {
    pub exp: String,
    pub replace: String,
    pub extension: bool,
}

pub struct RemoveData {
    pub first_n: usize,
    pub last_n: usize,
    pub start: usize,
    pub end: usize,
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

impl Default for RemoveData {
    fn default() -> Self {
        Self {
            first_n: 0,
            last_n: 0,
            start: 0,
            end: 0,
            characters: String::new(),
            words: String::new(),
            crop_before: true,
            crop: String::new(),
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
