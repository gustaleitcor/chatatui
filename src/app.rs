use std::{
    io::{stdout, Result},
    thread::{self, sleep},
    time::Duration,
};

use ratatui::{
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::Backend,
    Terminal,
};

use crate::{
    database::Database,
    pages::{chats::Chats, menu::Menu, messages::Messages, users::Users},
    state::State,
    ui::ui,
};
pub enum CursorMode {
    View(char),
    Edit(char),
}

pub enum FocusOn {
    Line(usize, usize),
    Filter(usize),
}

pub struct App {
    database: Database,
    state: State,
}

impl App {
    pub fn new() -> App {
        App {
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
        let mut chats = Chats::new();
        let mut messages = Messages::new();
        // self.database().load_users(1000);
        // self.database().load_chats(50);
        // self.database().load_messages(500);

        thread::spawn({
            let current_event = self.state.clone_current_event();
            move || loop {
                State::read_event(&current_event);
            }
        });
        loop {
            let now = std::time::Instant::now();

            terminal.draw(|f| {
                ui(f, self, &mut menu, &mut users, &mut messages, &mut chats);
            })?;

            if self.state().has_exited() {
                break;
            }

            sleep(Duration::from_millis(16).saturating_sub(now.elapsed()));

            self.state().error_timestamp().map(|timestamp| {
                if timestamp.elapsed().as_secs() >= 5 {
                    self.state_mut().clear_prompt_message();
                }
            });
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

impl CursorMode {
    pub fn as_str(&self) -> &str {
        match self {
            CursorMode::View('f') => "Filter",
            CursorMode::View(_) => "View",
            CursorMode::Edit('u') => "Update",
            CursorMode::Edit('c') => "Create",
            CursorMode::Edit(_) => "Edit",
        }
    }
}

impl Clone for FocusOn {
    fn clone(&self) -> Self {
        match self {
            FocusOn::Line(l, c) => FocusOn::Line(*l, *c),
            FocusOn::Filter(f) => FocusOn::Filter(*f),
        }
    }
}
