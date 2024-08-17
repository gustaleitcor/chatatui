use crate::ui::ui;
use std::{
    io::{stdout, Result},
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::Backend,
    style::Style,
    Terminal,
};

pub enum CurrentScreen {
    Register,
    Login,
    Chat,
    Exit,
}

pub enum CursorMode {
    Normal,
    Insert,
}

pub enum FocusOn {
    Username,
    Password,
}

pub struct App {
    current_event: Arc<Mutex<Option<Event>>>,
    current_screen: CurrentScreen,
    focus_on: Option<FocusOn>,
    cursor_mode: CursorMode,
    username: String,
    password: String,
    messages: Vec<String>,
    message: String,
}

impl App {
    pub fn new() -> App {
        App {
            current_event: Arc::new(Mutex::new(None)),
            current_screen: CurrentScreen::Login,
            username: String::new(),
            password: String::new(),
            messages: Vec::new(),
            message: String::new(),
            cursor_mode: CursorMode::Normal,
            focus_on: None,
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
        thread::spawn({
            let current_event = self.current_event.clone();
            move || loop {
                App::read_event(current_event.clone());
            }
        });
        loop {
            let now = std::time::Instant::now();

            terminal.draw(|f| {
                ui(f, self);
            })?;

            if let CurrentScreen::Exit = self.current_screen {
                break;
            }

            sleep(
                Duration::from_millis(16)
                    .checked_sub(now.elapsed())
                    .unwrap_or(Duration::ZERO),
            );
        }

        Ok(())
    }

    fn read_event(current_event: Arc<Mutex<Option<Event>>>) {
        let event = event::read().unwrap();
        current_event.lock().unwrap().replace(event);
    }

    pub fn current_screen(&self) -> &CurrentScreen {
        &self.current_screen
    }

    pub fn set_current_screen(&mut self, screen: CurrentScreen) {
        self.current_screen = screen;
    }

    pub fn take_current_event(&self) -> Option<Event> {
        self.current_event.lock().unwrap().take()
    }

    pub fn set_cursor_mode(&mut self, mode: CursorMode) {
        self.cursor_mode = mode;
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn pop_username(&mut self) {
        self.username.pop();
    }

    pub fn push_username(&mut self, c: char) {
        self.username.push(c);
    }

    pub fn pop_password(&mut self) {
        self.password.pop();
    }

    pub fn push_password(&mut self, c: char) {
        self.password.push(c);
    }

    pub fn focus_on(&self) -> Option<&FocusOn> {
        self.focus_on.as_ref()
    }

    pub fn set_focus_on(&mut self, focus_on: Option<FocusOn>) {
        self.focus_on = focus_on;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn cursor_mode(&self) -> &CursorMode {
        &self.cursor_mode
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn toggle_cursor_mode(&mut self) {
        self.cursor_mode = match self.cursor_mode {
            CursorMode::Normal => CursorMode::Insert,
            CursorMode::Insert => CursorMode::Normal,
        };
    }
}

impl CursorMode {
    pub fn as_str(&self) -> &str {
        match self {
            CursorMode::Normal => "Normal",
            CursorMode::Insert => "Insert",
        }
    }
}
