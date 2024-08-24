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
    pages::{menu::Menu, page::Page},
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

// db_cursor: i64,
// filter: Option<String>,
// users: Vec<User>,
// new_user: User,
// messages: Vec<Message>,
// chats: Vec<Chat>,

pub struct Admin {
    pg_conn: PgConnection,
    state: State,
}

impl Admin {
    pub fn new() -> Admin {
        Admin {
            pg_conn: establish_connection(),
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

        thread::spawn({
            let current_event = self.state.clone_current_event();
            move || loop {
                State::read_event(&current_event);
            }
        });
        loop {
            let now = std::time::Instant::now();

            terminal.draw(|f| {
                ui_admin(f, self, &mut menu);
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

    // returns the number of users fetched
    // pub fn fetch_users(&mut self, limit: i64) -> usize {
    //     let db_cursor = self.db_cursor();
    //     let filter = self.filter().to_owned();

    //     self.users = match crud_bd::crud::user::get_users_with_pagination(
    //         self.pg_conn(),
    //         db_cursor,
    //         limit,
    //         filter,
    //     ) {
    //         Ok(users) => {
    //             if users.is_empty() {
    //                 return 0;
    //             }
    //             users
    //         }
    //         Err(err) => {
    //             self.set_prompt_message(Some(Err(std::io::Error::new(
    //                 std::io::ErrorKind::Other,
    //                 format!("Failed to fetch user. {:?}", err.to_string()),
    //             ))));

    //             self.set_db_cursor(0);
    //             Vec::new()
    //         }
    //     };

    //     self.users.len()
    // }

    // pub fn next_users_page(&mut self, limit: i64) -> usize {
    //     self.set_db_cursor(self.db_cursor().saturating_add(limit));

    //     let n = self.fetch_users(limit);

    //     self.set_db_cursor(self.db_cursor + n as i64 - limit);

    //     n
    // }

    // pub fn prev_users_page(&mut self, limit: i64) -> usize {
    //     if self.db_cursor - limit < 0 {
    //         self.set_db_cursor(0);
    //     } else {
    //         self.set_db_cursor(self.db_cursor() - limit);
    //     }

    //     let n = self.fetch_users(limit);

    //     if self.db_cursor - limit < 0 {
    //         self.set_db_cursor(0);
    //     } else {
    //         self.set_db_cursor(self.db_cursor - n as i64 + limit);
    //     }

    //     n
    // }
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
