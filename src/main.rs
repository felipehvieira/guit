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
use git_app::get_repository;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    //async function (verify tokio channel to send status state for ui)
    get_repository();
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
