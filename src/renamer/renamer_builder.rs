use super::date;
use super::*;

#[derive(Default, Debug)]
pub struct RenamerBuilder {
    renamer: Renamer,
}

impl RenamerBuilder {
    pub fn build(self) -> Renamer {
        self.renamer
    }

    pub fn with_add(
        mut self,
        prefix: Option<String>,
        insert: Option<(i32, String)>,
        suffix: Option<String>,
        word_space: bool,
    ) -> Self {
        self.renamer.add = Some(AddOptions {
            prefix,
            insert,
            suffix,
            word_space,
        });
        Self {
            renamer: self.renamer,
        }
    }

    pub fn with_case(mut self, case: Case, snake: bool, exceptions: String) -> Self {
        self.renamer.case = Some(CaseOptions {
            case,
            snake,
            exceptions,
        });
        Self {
            renamer: self.renamer,
        }
    }

    pub fn with_date(
        mut self,
        date_mode: date::DateMode,
        date_type: date::DateType,
        fmt: date::DateFormat,
        sep: String,
        seg: String,
        full_year: bool,
    ) -> Self {
        self.renamer.date = Some(DateOptions {
            date_mode,
            date_type,
            fmt,
            sep,
            seg,
            full_year,
        });
        Self {
            renamer: self.renamer,
        }
    }

    pub fn with_extension(mut self, option: ExtensionOptions) -> Self {
        self.renamer.ext = Some(option);
        Self {
            renamer: self.renamer,
        }
    }

    pub fn with_folder(mut self, option: FolderOptions) -> Self {
        self.renamer.folder = Some(option);
        Self {
            renamer: self.renamer,
        }
    }

    pub fn with_name(mut self, option: NameOptions) -> Self {
        self.renamer.name = Some(option);
        Self {
            renamer: self.renamer,
        }
    }

    pub fn with_number(mut self, option: NumberOptions) -> Self {
        self.renamer.number = Some(option);
        Self {
            renamer: self.renamer,
        }
    }

    pub fn with_reg(mut self, option: RegexOptions) -> Self {
        self.renamer.regex = Some(option);
        Self {
            renamer: self.renamer,
        }
    }

    pub fn with_remove(mut self, option: RemoveOptions) -> Self {
        self.renamer.remove = Some(option);
        Self {
            renamer: self.renamer,
        }
    }

    pub fn with_replace(mut self, option: RegexOptions) -> Self {
        self.renamer.replace = Some(option);
        Self {
            renamer: self.renamer,
        }
    }
}
