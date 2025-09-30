use renameit_gui::{get_initial_directory, run};
use renameit_lib::{FileError, Renamer, RenamerError};

fn main() -> Result<(), RenamerError> {
    let mut files = Selected::default();
    if let Ok(initial) = get_initial_directory(std::env::args().nth(1)) {
        files.add(initial)?;
    };
    match run() {
        Ok(_) => Ok(()),
        Err(e) => {
            files.clear();
            Err(e.into())
        }
    }
}

use std::path::PathBuf;

#[derive(Debug, Default)]
struct Selected {
    selected: Vec<Renamer>,
}

impl Selected {
    fn clear(&mut self) {
        self.selected.clear()
    }

    fn add(&mut self, file: PathBuf) -> Result<(), FileError> {
        self.selected.push(Renamer::try_from(file.as_path())?);
        Ok(())
    }
}
