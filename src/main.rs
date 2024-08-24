mod admin;
mod database;
mod pages;
mod state;
mod ui_admin;

use admin::Admin;
// use app::App;
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // let mut app = App::new();

    // app.setup()?;

    // app.run(&mut terminal)?;

    // app.cleanup()?;

    let mut admin = Admin::new();

    admin.setup()?;

    admin.run(&mut terminal)?;

    admin.cleanup()?;

    Ok(())
}
