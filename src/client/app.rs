use crate::ui::ui;
use std::{
    io::{stdout, Result},
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use diesel::pg::PgConnection;

use crud_bd::crud::{establish_connection, user::User};
use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::Backend,
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
    Input,
    Message(usize),
}

pub struct App {
    current_event: Arc<Mutex<Option<Event>>>,
    current_screen: CurrentScreen,
    focus_on: Option<FocusOn>,
    cursor_mode: CursorMode,
    messages: Vec<String>,
    message: String,
    error: String,
    user: User,
    pg_conn: PgConnection,
}

impl App {
    pub fn new() -> App {
        App {
            current_event: Arc::new(Mutex::new(None)),
            current_screen: CurrentScreen::Login,
            user: User {
                id: 0,
                username: String::new(),
                password: String::new(),
            },
            messages: Vec::new(),
            message: String::new(),
            cursor_mode: CursorMode::Normal,
            focus_on: None,
            error: String::new(),
            pg_conn: establish_connection(),
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
        self.user.username = username;
    }

    pub fn pop_username(&mut self) {
        self.user.username.pop();
    }

    pub fn push_username(&mut self, c: char) {
        self.user.username.push(c);
    }

    pub fn pop_password(&mut self) {
        self.user.password.pop();
    }

    pub fn push_password(&mut self, c: char) {
        self.user.password.push(c);
    }

    pub fn focus_on(&self) -> Option<&FocusOn> {
        self.focus_on.as_ref()
    }

    pub fn set_focus_on(&mut self, focus_on: Option<FocusOn>) {
        self.focus_on = focus_on;
    }

    pub fn set_password(&mut self, password: String) {
        self.user.password = password;
    }

    pub fn cursor_mode(&self) -> &CursorMode {
        &self.cursor_mode
    }

    pub fn password(&self) -> &str {
        &self.user.password
    }

    pub fn username(&self) -> &str {
        &self.user.username
    }

    pub fn toggle_cursor_mode(&mut self) {
        self.cursor_mode = match self.cursor_mode {
            CursorMode::Normal => CursorMode::Insert,
            CursorMode::Insert => CursorMode::Normal,
        };
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn push_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &Vec<String> {
        &self.messages
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn pop_message(&mut self) {
        self.message.pop();
    }

    pub fn push_message_char(&mut self, c: char) {
        self.message.push(c);
    }

    pub fn set_error(&mut self, error: String) {
        self.error = error;
    }

    pub fn error(&self) -> &String {
        &self.error
    }

    pub fn pg_conn(&mut self) -> &mut PgConnection {
        &mut self.pg_conn
    }

    pub fn set_pg_conn(&mut self, pg_conn: PgConnection) {
        self.pg_conn = pg_conn;
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn set_user(&mut self, user: User) {
        self.user = user;
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
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
