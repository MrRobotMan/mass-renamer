use crate::{Process, RenameFile};
use std::{
    cmp::min,
    env,
    fmt::Write,
    path::{Component, Prefix},
};

/// Add the name of the containing folder or hierarchy of folders.
/// These can be added in prefix or suffix `Mode`, with a `Sep`arator specified and the
/// maximum number of `Levels` selected.
///
/// On Windows, if the hierarchy reaches the drive root (i.e. C:\ on windows, \\ on linux)
/// the ":\" or "\\"characters will be automatically removed.
pub struct FolderOptions<'a> {
    pub mode: FolderMode,
    pub sep: &'a str,
    pub levels: i32,
}

impl Process for FolderOptions<'_> {
    fn process(&self, file: &mut RenameFile) {
        let mut parts = file.original.components().rev();
        parts.next(); // Skip the file itself.
        let components: Vec<_> = parts
            .filter_map(|p| match p {
                Component::Normal(s) => Some(s.to_str()),
                Component::Prefix(prefix) => match prefix.kind() {
                    Prefix::Verbatim(s) => Some(s.to_str()),
                    Prefix::VerbatimUNC(_, s) => Some(s.to_str()),
                    Prefix::VerbatimDisk(_) => Some(prefix.as_os_str().to_str()),
                    Prefix::DeviceNS(s) => Some(s.to_str()),
                    Prefix::UNC(_, s) => Some(s.to_str()),
                    Prefix::Disk(_) => Some(prefix.as_os_str().to_str()),
                },
                _ => None,
            })
            .collect();
        let end = min(components.len(), self.levels.unsigned_abs() as usize);
        let start = if self.levels >= 0 { 0 } else { end - 1 };
        match self.mode {
            FolderMode::Prefix => {
                for component in components[start..end].iter().flatten() {
                    let mut component = component.replace(r"\\?\", "");
                    if env::consts::OS == "windows" {
                        component = component.replace(':', "")
                    }
                    file.stem
                        .insert_str(0, &format!("{}{}", component, self.sep));
                }
            }
            FolderMode::Suffix => {
                for component in components[start..end].iter().flatten() {
                    let mut component = component.replace(r"\\?\", "");
                    if env::consts::OS == "windows" {
                        component = component.replace(':', "")
                    }
                    write!(file.stem, "{}{}", component, self.sep)
                        .expect("Unexpected error appending string.")
                }
            }
        };
    }
}

/// Select from
/// `FolderMode::Prefix` or
/// `FolderMode::Suffix`.
#[derive(Default)]
pub enum FolderMode {
    #[default]
    Prefix,
    Suffix,
}

#[cfg(test)]
mod folder_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn prefix_two_levels() {
        let mut file = RenameFile::new(Path::new("/some/file/path/to/test file.txt")).unwrap();
        let mode = FolderMode::Prefix;
        let sep = "~";
        let levels = 2;
        let opt = FolderOptions { mode, sep, levels };
        opt.process(&mut file);
        assert_eq!(file.stem, "path~to~test file".to_string())
    }

    #[test]
    fn suffix_negative_two_levels() {
        let mut file =
            RenameFile::new(Path::new(r"\\?\c:\some\file\path\to\test file.txt")).unwrap();
        let mode = FolderMode::Prefix;
        let sep = "~";
        let levels = -2;
        let opt = FolderOptions { mode, sep, levels };
        opt.process(&mut file);
        assert_eq!(file.stem, "path~test file".to_string())
    }
}
