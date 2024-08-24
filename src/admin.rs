use std::{
    io::{stdout, Result},
    ptr::null,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use crud_bd::crud::{chat::Chat, establish_connection, message::Message, user::User};
use diesel::PgConnection;
use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::Backend,
    Frame, Terminal,
};

use crate::{
    database::Database,
    pages::{menu::Menu, page::Page, users::Users},
    state::State,
    ui_admin::ui_admin,
};
pub enum AdminCursorMode {
    View(char),
    Edit(char),
}

pub enum AdminFocusOn {
    Line(usize, usize),
}

pub struct Admin {
    database: Database,
    state: State,
}

impl Admin {
    pub fn new() -> Admin {
        Admin {
            database: Database::new(),
            state: State::new(),
        }
    }

    pub fn setup(&self) -> Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        stdout().execute(EnableMouseCapture)?;
        Ok(())
    }

    pub fn cleanup(&self) -> Result<()> {
        stdout().execute(DisableMouseCapture)?;
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let mut menu = Menu::new();
        let mut users = Users::new();

        thread::spawn({
            let current_event = self.state.clone_current_event();
            move || loop {
                State::read_event(&current_event);
            }
        });
        loop {
            let now = std::time::Instant::now();

            terminal.draw(|f| {
                ui_admin(f, self, &mut menu, &mut users);
            })?;

            if self.state().has_exited() {
                break;
            }

            sleep(Duration::from_millis(16).saturating_sub(now.elapsed()));
        }

        Ok(())
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn database(&mut self) -> &mut Database {
        &mut self.database
    }
}

impl AdminCursorMode {
    pub fn as_str(&self) -> &str {
        match self {
            AdminCursorMode::View('f') => "Filter",
            AdminCursorMode::View(_) => "View",
            AdminCursorMode::Edit('u') => "Update",
            AdminCursorMode::Edit('c') => "Create",
            AdminCursorMode::Edit(_) => "Edit",
        }
    }
}

impl Clone for AdminFocusOn {
    fn clone(&self) -> Self {
        match self {
            AdminFocusOn::Line(l, c) => AdminFocusOn::Line(*l, *c),
        }
    }
}
