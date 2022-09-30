use crate::file::{Process, RenameFile};
use std::{cmp::min, path::Component};

/// Add the name of the containing folder or hierarchy of folders.
/// These can be added in prefix or suffix `Mode`, with a `Sep`arator specified and the
/// maximum number of `Levels` selected.
///
/// On Windows, if the hierarchy reaches the drive root (i.e. C:\\) the ":\\"
/// characters will be automatically removed.
pub struct FolderOptions<'a> {
    pub mode: FolderMode,
    pub sep: &'a str,
    pub levels: i32,
}

impl Process for FolderOptions<'_> {
    fn process(&self, file: &mut RenameFile) {
        let mut components = file.original.components().rev();
        components.next(); // Skip the file itself.

        let components: Vec<_> = components
            .filter_map(|p| match p {
                Component::Normal(s) => Some(s.to_str()),
                _ => None,
            })
            .collect();
        let end = min(components.len(), self.levels.abs() as usize);
        let start = if self.levels >= 0 { 0 } else { end - 1 };
        match self.mode {
            FolderMode::Prefix => {
                for idx in start..end {
                    if let Some(s) = components[idx] {
                        file.stem.insert_str(0, &format!("{}{}", s, self.sep))
                    };
                }
            }
            FolderMode::Suffix => {
                for idx in start..end {
                    if let Some(s) = components[idx] {
                        file.stem.push_str(&format!("{}{}", s, self.sep))
                    }
                }
            }
        };
    }
}

/// Select from
/// `FolderMode::Prefix` or
/// `FolderMode::Suffix`.
pub enum FolderMode {
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
        let mut file = RenameFile::new(Path::new("/some/file/path/to/test file.txt")).unwrap();
        let mode = FolderMode::Prefix;
        let sep = "~";
        let levels = -2;
        let opt = FolderOptions { mode, sep, levels };
        opt.process(&mut file);
        assert_eq!(file.stem, "path~test file".to_string())
    }
}
