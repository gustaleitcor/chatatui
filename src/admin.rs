use std::{
    io::{stdout, Result},
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
    Terminal,
};

use crate::{
    app::{CurrentScreen, CursorMode, FocusOn},
    ui_admin::{self, ui_admin},
};
pub enum AdminCursorMode {
    View,
    Edit(char),
}

pub enum AdminFocusOn {
    Line(usize, usize),
}

pub enum AdminCurrentScreen {
    Menu,
    Users,
    Messages,
    Chats,
    Exit,
}

pub struct Admin {
    current_event: Arc<Mutex<Option<Event>>>,
    current_screen: AdminCurrentScreen,
    focus_on: Option<AdminFocusOn>,
    cursor_mode: AdminCursorMode,
    error: Option<String>,
    pg_conn: PgConnection,
    db_cursor: i64,
    input: String,
    users: Vec<User>,
    user: User,
    messages: Vec<Message>,
    chats: Vec<Chat>,
}

impl Admin {
    pub fn new() -> Admin {
        Admin {
            current_event: Arc::new(Mutex::new(None)),
            current_screen: AdminCurrentScreen::Menu,
            cursor_mode: AdminCursorMode::View,
            focus_on: None,
            error: None,
            pg_conn: establish_connection(),
            db_cursor: 0,
            input: String::new(),
            users: Vec::new(),
            messages: Vec::new(),
            chats: Vec::new(),
            user: User {
                id: 0,
                username: String::new(),
                password: String::new(),
            },
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
                Admin::read_event(current_event.clone());
            }
        });
        loop {
            let now = std::time::Instant::now();

            terminal.draw(|f| {
                ui_admin(f, self);
            })?;

            if let AdminCurrentScreen::Exit = self.current_screen {
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

    pub fn current_screen(&self) -> &AdminCurrentScreen {
        &self.current_screen
    }

    pub fn set_current_screen(&mut self, screen: AdminCurrentScreen) {
        self.current_screen = screen;
    }

    pub fn take_current_event(&self) -> Option<Event> {
        self.current_event.lock().unwrap().take()
    }

    pub fn toggle_cursor_mode(&mut self) {
        self.cursor_mode = match self.cursor_mode {
            AdminCursorMode::View => AdminCursorMode::Edit('x'),
            AdminCursorMode::Edit(_) => AdminCursorMode::View,
        };
    }

    pub fn set_cursor_mode(&mut self, mode: AdminCursorMode) {
        self.cursor_mode = mode;
    }

    pub fn focus_on(&self) -> &Option<AdminFocusOn> {
        &self.focus_on
    }

    pub fn set_focus_on(&mut self, focus_on: Option<AdminFocusOn>) {
        self.focus_on = focus_on;
    }

    pub fn cursor_mode(&self) -> &AdminCursorMode {
        &self.cursor_mode
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn error(&self) -> &Option<String> {
        &self.error
    }

    pub fn pg_conn(&mut self) -> &mut PgConnection {
        &mut self.pg_conn
    }

    pub fn get_user(&self) -> &User {
        &self.user
    }

    pub fn set_user(&mut self, user: User) {
        self.user = user;
    }

    pub fn pop_user_username(&mut self) -> Option<char> {
        self.user.username.pop()
    }

    pub fn push_user_username(&mut self, c: char) {
        self.user.username.push(c);
    }

    pub fn pop_user_password(&mut self) -> Option<char> {
        self.user.password.pop()
    }

    pub fn push_user_password(&mut self, c: char) {
        self.user.password.push(c);
    }

    pub fn set_pg_conn(&mut self, pg_conn: PgConnection) {
        self.pg_conn = pg_conn;
    }

    pub fn input(&self) -> &String {
        &self.input
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;
    }

    pub fn users(&self) -> &Vec<User> {
        &self.users
    }

    pub fn users_mut(&mut self) -> &mut Vec<User> {
        &mut self.users
    }

    pub fn set_users(&mut self, users: Vec<User>) {
        self.users = users;
    }

    pub fn db_cursor(&self) -> i64 {
        self.db_cursor
    }

    pub fn set_db_cursor(&mut self, db_cursor: i64) {
        self.db_cursor = db_cursor;
    }

    // returns the number of users fetched
    pub fn fetch_users(&mut self, limit: i64) -> usize {
        let db_cursor = self.db_cursor();

        self.users = match crud_bd::crud::user::get_users_with_pagination(
            self.pg_conn(),
            db_cursor,
            limit,
        ) {
            Ok(users) => {
                if users.is_empty() {
                    return 0;
                }
                users
            }
            Err(e) => {
                self.set_error(e.to_string());
                self.set_db_cursor(0);
                Vec::new()
            }
        };

        self.users.len()
    }

    pub fn next_users_page(&mut self, limit: i64) -> usize {
        self.set_db_cursor(self.db_cursor().saturating_add(limit));

        let n = self.fetch_users(limit);

        self.set_db_cursor(self.db_cursor + n as i64 - limit);

        n
    }

    pub fn prev_users_page(&mut self, limit: i64) -> usize {
        if self.db_cursor - limit < 0 {
            self.set_db_cursor(0);
        } else {
            self.set_db_cursor(self.db_cursor() - limit);
        }

        let n = self.fetch_users(limit);

        self.set_db_cursor(self.db_cursor - n as i64 + limit);
        n
    }
}

impl AdminCursorMode {
    pub fn as_str(&self) -> &str {
        match self {
            AdminCursorMode::View => "View",
            AdminCursorMode::Edit('e') => "Update",
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
