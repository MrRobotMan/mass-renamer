use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use bulk_file_renamer::{get_directory, DirectoryError, Files};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use thiserror::Error;
use tui::{backend::CrosstermBackend, Terminal};

const TICK_RATE: Duration = Duration::from_millis(200);

fn main() -> Result<(), RenamerError> {
    let terminal = initialize()?;

    let _initial = get_directory(std::env::args().nth(1))?;
    let mut _files = Files::default();
    // let mut cur = initial_dir.into();

    loop {
        if event::poll(TICK_RATE)? {
            match event::read()? {
                Event::Key(key) if matches!(key.kind, event::KeyEventKind::Press) => {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Tab => todo!(),
                        KeyCode::Enter => todo!(),
                        KeyCode::Up => todo!(),
                        KeyCode::Down => todo!(),
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }
    }
    shutdown(terminal)
}

fn initialize() -> Result<Terminal<CrosstermBackend<Stdout>>, RenamerError> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn shutdown(mut term: Terminal<CrosstermBackend<Stdout>>) -> Result<(), RenamerError> {
    terminal::disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen)?;
    term.show_cursor()?;
    term.clear()?;
    Ok(())
}

/*
fn disabled() {
    loop {
        println!("\n0. ..");
        for (idx, file) in cur.read_dir().unwrap().enumerate() {
            println!(
                "{}. {}",
                idx + 1,
                file.unwrap().path().file_name().unwrap().to_string_lossy()
            );
        }
        let selected = get_input();
        if selected == 0 {
            files.clear();
            cur = PathBuf::from(cur.parent().unwrap());
        } else {
            match cur.read_dir().unwrap().nth(selected - 1) {
                None => println!("Invalid entry. Try again"),
                Some(f) => {
                    let f = f.unwrap().path();
                    match f.is_dir() {
                        true => {
                            files.clear();
                            cur = f;
                        }
                        false => {
                            files.add(f);
                        }
                    }
                }
            }
        }
    }
}
*/

#[derive(Debug, Error)]
enum RenamerError {
    #[error(transparent)]
    Directory(#[from] DirectoryError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
