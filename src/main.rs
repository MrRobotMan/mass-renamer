use mass_renamer::{app, get_initial_directory, gui};

fn main() -> Result<(), app::RenamerError> {
    let mut files = app::Selected::default();
    if let Ok(initial) = get_initial_directory(std::env::args().nth(1)) {
        files.add(initial)?;
    };
    match gui::run() {
        Ok(_) => Ok(()),
        Err(e) => {
            files.clear();
            Err(e.into())
        }
    }
}
