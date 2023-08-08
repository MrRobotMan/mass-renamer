use std::{
    cmp::Ordering,
    fmt::Debug,
    path::{Path, PathBuf},
};

mod add;
mod case;
mod date;
mod extension;
mod folder;
mod name;
mod number;
mod reg;
mod remove;
mod replace;

use crate::generate_path_as_string;
pub use add::AddOptions;
pub use case::{Case, CaseOptions};
pub use date::{DateFormat, DateMode, DateOptions, DatePrefix, DateSuffix, DateType};
pub use extension::ExtensionOptions;
pub use folder::{FolderMode, FolderOptions};
pub use name::NameOptions;
pub use number::{NumberFormat, NumberMode, NumberOptions};
pub use reg::RegexOptions;
pub use remove::RemoveOptions;
pub use replace::ReplaceOptions;

#[derive(Debug, Default)]
pub struct File {
    stem: String,
    extension: Option<String>,
    original: PathBuf,
    add: Option<AddOptions>,
    case: Option<CaseOptions>,
    date: Option<DateOptions>,
    ext: Option<ExtensionOptions>,
    folder: Option<FolderOptions>,
    name: Option<NameOptions>,
    number: Option<NumberOptions>,
    regex: Option<RegexOptions>,
    remove: Option<RemoveOptions>,
    replace: Option<RegexOptions>,
}

impl File {
    pub fn new(path: &Path) -> Option<Self> {
        // if !path.is_file() {
        //     return None;
        // }
        let extension = generate_path_as_string(path.extension());
        let stem = generate_path_as_string(path.file_stem())?;
        Some(Self {
            stem,
            extension,
            original: path.to_owned(),
            ..Default::default()
        })
    }

    /// Tool to rename a single file.
    /// Takes the `&path` and various options (processed in order) to return a `PathBuf`
    /// used to rename the file.
    /// Options are
    ///    -  1 RegEx
    ///    -  2 Name
    ///    -  3 Replace
    ///    -  4 Case
    ///    -  5 Remove
    ///    -  6 Add
    ///    -  7 Auto Date
    ///    -  8 Append Folder Name
    ///    -  9 Numbering
    ///    - 10 Extension
    ///
    /// # Example
    ///
    /// ```
    /// # use std::path::{Path, PathBuf};
    /// # use bulk_file_renamer::file::{NameOptions, Case, CaseOptions, File, Process, Options};
    /// let file = Path::new("file.txt");
    /// let name = NameOptions::Fixed("new_name".into());
    /// let case = CaseOptions{case: Case::Upper, snake: false, exceptions: Some("n".into())};
    /// let mut rename = File::new(file).unwrap().with_option(Options::Name(name)).with_option(Options::Case(case));
    /// let new_name = rename.rename();
    /// assert_eq!(new_name, PathBuf::from("nEW_nAME.txt"));
    /// ```
    pub fn rename(&mut self) -> PathBuf {
        let mut opts: Vec<Box<dyn Process>> = vec![];
        if let Some(opt) = &self.regex {
            opts.push(Box::new(opt.clone()));
        };
        if let Some(opt) = &self.name {
            opts.push(Box::new(opt.clone()));
        };
        if let Some(opt) = &self.replace {
            opts.push(Box::new(opt.clone()));
        };
        if let Some(opt) = &self.case {
            opts.push(Box::new(opt.clone()));
        };
        if let Some(opt) = &self.remove {
            opts.push(Box::new(opt.clone()));
        };
        if let Some(opt) = &self.add {
            opts.push(Box::new(opt.clone()));
        };
        if let Some(opt) = &self.date {
            opts.push(Box::new(opt.clone()));
        };
        if let Some(opt) = &self.folder {
            opts.push(Box::new(opt.clone()));
        };
        if let Some(opt) = &self.number {
            opts.push(Box::new(opt.clone()));
        };
        if let Some(opt) = &self.ext {
            opts.push(Box::new(opt.clone()));
        };
        for opt in opts {
            opt.process(self);
        }
        let mut new_name = match self.original.parent() {
            None => PathBuf::from("/"),
            Some(p) => PathBuf::from(p),
        };
        new_name.push(Path::new(&self.stem));
        match &self.extension {
            None => new_name,
            Some(e) => new_name.with_extension(e),
        }
    }

    pub fn with_option(mut self, option: Options) -> Self {
        use Options::*;
        match option {
            Regex(opt) => self.regex = Some(opt),
            Name(opt) => self.name = Some(opt),
            Case(opt) => self.case = Some(opt),
            Remove(opt) => self.remove = Some(opt),
            Add(opt) => self.add = Some(opt),
            Date(opt) => self.date = Some(opt),
            Folder(opt) => self.folder = Some(opt),
            Number(opt) => self.number = Some(opt),
            Extension(opt) => self.ext = Some(opt),
        }
        self
    }
}

#[derive(Debug)]
pub enum Options {
    Regex(RegexOptions),
    Name(NameOptions),
    Case(CaseOptions),
    Remove(RemoveOptions),
    Add(AddOptions),
    Date(DateOptions),
    Folder(FolderOptions),
    Number(NumberOptions),
    Extension(ExtensionOptions),
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.original.is_dir(), other.original.is_dir()) {
            (true, true) => self.stem.cmp(&other.stem),
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => match (&self.extension, &other.extension) {
                (None, None) => self.stem.cmp(&other.stem),
                (None, Some(ext)) => {
                    let mut rhs = other.stem.clone();
                    rhs.push_str(ext);
                    self.stem.cmp(&rhs)
                }
                (Some(ext), None) => {
                    let mut lhs = self.stem.clone();
                    lhs.push_str(ext);
                    lhs.cmp(&other.stem)
                }
                (Some(self_ext), Some(other_ext)) => {
                    let mut lhs = self.stem.clone();
                    lhs.push_str(self_ext);
                    let mut rhs = other.stem.clone();
                    rhs.push_str(other_ext);
                    lhs.cmp(&rhs)
                }
            },
        }
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.original == other.original
    }
}

impl Eq for File {}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait Process {
    fn process(&self, file: &mut File);
}

#[cfg(test)]
mod file_tests {
    use super::*;

    #[test]
    fn test_regex() {
        let file = Path::new("Testfile123.txt");
        let expected = PathBuf::from("TestfileABC.txt");
        let opt = RegexOptions {
            exp: "123".into(),
            rep: "ABC".into(),
            extension: false,
        };
        let mut rename = File::new(file).unwrap().with_option(Options::Regex(opt));
        let result = rename.rename();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_name() {
        let file = Path::new("file.txt");
        let expected = PathBuf::from("new_name.txt");
        let name = NameOptions::Fixed("new_name".into());
        let mut rename = File::new(file).unwrap().with_option(Options::Name(name));
        let new_name = rename.rename();
        assert_eq!(new_name, expected)
    }
}
