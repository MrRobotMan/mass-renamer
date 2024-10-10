use mass_renamer::{file::directory::get_initial_directory, gui, RenamerError, Selected};

fn main() -> Result<(), RenamerError> {
    let _initial = get_initial_directory(std::env::args().nth(1))?;
    let mut _files = Selected::default();
    Ok(gui::run()?)
}
