use std::{error::Error, io};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use git2::{Repository, Status, StatusOptions};
use ratatui::prelude::{CrosstermBackend, Terminal};
mod app;
mod git_app;
pub use app::*;
use git_app::*;
fn main2() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new().run(terminal);

    // restore terminal
    disable_raw_mode()?;
    io::stdout().execute(DisableMouseCapture)?;
    io::stdout().execute(LeaveAlternateScreen)?;

    if let Err(err) = app {
        println!("{err:?}");
    }
    Ok(())
}
fn main() {
    let _ = get_repository();
}
pub fn get_repository() -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    if repo.is_bare() {
        return Err(git2::Error::from_str("Erro"));
    }
    let mut opts = StatusOptions::new();
    let statuses = repo.statuses(Some(&mut opts))?;
    for entry in statuses.iter() {
        let file = entry.index_to_workdir().unwrap().old_file().path().unwrap();
        println!("{}", file.display())
    }
    return Ok(());
}
