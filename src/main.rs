mod admin;
mod app;
mod client;
mod common;
mod database;
mod page;
mod state;
mod ui;

use app::App;
// use crate::client::app::App;
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    app.setup()?;

    app.run(&mut terminal)?;

    app.cleanup()?;

    Ok(())
}
