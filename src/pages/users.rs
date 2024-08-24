use std::{
    io::{Result, Stdout},
    rc::Rc,
};

use crud_bd::crud::user::User;
use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    Frame,
};

use crate::{
    admin::{Admin, AdminCursorMode, AdminFocusOn},
    state::State,
};

use super::page::Page;

pub struct Users {
    chunks: Rc<[Rect]>,
    db_cursor: i64,
    filter: Option<String>,
    users: Vec<User>,
    new_user: User,
    available_rows: u16,
}

impl Users {
    pub fn new() -> Self {
        Self {
            chunks: Rc::new([Rect::default()]),
            db_cursor: 0,
            filter: None,
            users: Vec::new(),
            new_user: User {
                id: -1,
                username: String::new(),
                password: String::new(),
            },
            available_rows: 0,
        }
    }
}

impl Page<CrosstermBackend<Stdout>> for Users {
    fn render(&self, frame: &mut Frame, state: &mut State) -> Result<()> {
        // render users
        // render new user form
        // render pagination
        Ok(())
    }
    fn setup(&mut self, frame: &mut Frame) -> Result<()> {
        self.chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Fill(1),
                    Constraint::Length(2),
                    Constraint::Length(2),
                ]
                .as_ref(),
            )
            .split(frame.size());

        self.available_rows = self.chunks[1].height.saturating_sub(1);

        Ok(())
    }
    fn handle_input(&mut self, app: &mut Admin, key: &KeyEvent) -> Result<()> {
        if let AdminCursorMode::Edit('c') = app.state().cursor_mode() {
            return Ok(());
        }

        if let Some(Event::Key(key)) = current_event {
            match app.cursor_mode() {
                AdminCursorMode::View('x') => match key.code {
                    KeyCode::Char('q') => {
                        app.set_focus_on(Some(AdminFocusOn::Line(0, 1)));
                        app.set_current_screen(AdminCurrentScreen::Menu);
                    }
                    KeyCode::Down => {
                        if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
                            if !app.users().is_empty() {
                                if *n < app.users().len() - 1 {
                                    app.set_focus_on(Some(AdminFocusOn::Line(n + 1, 1)));
                                } else {
                                    app.next_users_page(n_rows);
                                    app.set_focus_on(Some(AdminFocusOn::Line(0, 1)));
                                }
                            } else {
                                app.set_focus_on(None);
                            }
                        }
                    }

                    KeyCode::Char('f') => {

                        // if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
                        //     if !app.users().is_empty() {
                        //         if *n < app.users().len() - 1 {
                        //             app.set_focus_on(Some(AdminFocusOn::Line(n + 1, 1)));
                        //         } else {
                        //             app.next_users_page(
                        //                 chunks[1].height.saturating_sub(1) as i64
                        //             );
                        //             app.set_focus_on(Some(AdminFocusOn::Line(0, 1)));
                        //         }
                        //     } else {
                        //         app.set_focus_on(None);
                        //     }
                        // }
                    }

                    KeyCode::Up => {
                        if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
                            if !app.users().is_empty() {
                                if *n > 0 {
                                    app.set_focus_on(Some(AdminFocusOn::Line(n - 1, 1)));
                                } else {
                                    app.prev_users_page(n_rows);
                                    app.set_focus_on(Some(AdminFocusOn::Line(
                                        app.users().len() - 1,
                                        1,
                                    )));
                                }
                            } else {
                                app.set_focus_on(None);
                            }
                        }
                    }

                    _ => {}
                },

                AdminCursorMode::Edit('x') => match key.code {
                    KeyCode::Char('d') => {
                        if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
                            let n = *n;
                            if !app.users().is_empty() {
                                let user = app.users().get(n).unwrap();
                                let user_id = user.id.to_owned();
                                match crud_bd::crud::user::delete_user(app.pg_conn(), user_id) {
                                    Ok(_) => {
                                        app.users_mut().remove(n);
                                        app.set_focus_on(Some(AdminFocusOn::Line(
                                            n.saturating_sub(1),
                                            1,
                                        )));
                                        app.set_prompt_message(Some(Ok("User deleted".to_string())))
                                    }

                                    Err(err) => {
                                        app.set_prompt_message(Some(Err(std::io::Error::new(
                                            std::io::ErrorKind::Other,
                                            format!("Failed to delete user. {:?}", err),
                                        ))))
                                    }
                                }
                            }
                        }
                    }

                    KeyCode::Char('u') => {
                        if !app.users().is_empty() {
                            if let Some(AdminFocusOn::Line(row, _)) = app.focus_on() {
                                if app.users().get(*row).is_some() {
                                    app.set_cursor_mode(AdminCursorMode::Edit('u'));
                                }
                            }
                        }
                    }

                    KeyCode::Char('c') => {
                        app.set_cursor_mode(AdminCursorMode::Edit('c'));
                        app.users_mut().push(User {
                            id: -1,
                            username: "".to_string(),
                            password: "".to_string(),
                        });
                        app.set_focus_on(Some(AdminFocusOn::Line(app.users().len() - 1, 1)));
                    }

                    KeyCode::Down => {
                        if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
                            if !app.users().is_empty() {
                                if *n < app.users().len() - 1 {
                                    app.set_focus_on(Some(AdminFocusOn::Line(n + 1, 1)));
                                } else {
                                    app.next_users_page(n_rows);
                                    app.set_focus_on(Some(AdminFocusOn::Line(0, 1)));
                                }
                            } else {
                                app.set_focus_on(None);
                            }
                        }
                    }

                    KeyCode::Up => {
                        if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
                            if !app.users().is_empty() {
                                if *n > 0 {
                                    app.set_focus_on(Some(AdminFocusOn::Line(n - 1, 1)));
                                } else {
                                    app.prev_users_page(n_rows);
                                    app.set_focus_on(Some(AdminFocusOn::Line(
                                        app.users().len() - 1,
                                        1,
                                    )));
                                }
                            } else {
                                app.set_focus_on(None);
                            }
                        }
                    }

                    _ => {}
                },

                AdminCursorMode::Edit('u') => {
                    if let Some(AdminFocusOn::Line(row, col)) = app.focus_on().clone() {
                        if let Some(Event::Key(key)) = current_event {
                            match key.code {
                                KeyCode::Enter => {
                                    if let Some(user) = app.users_mut().get_mut(row) {
                                        let user_id = &user.id.to_owned();
                                        let user_name = &user.username.to_owned();
                                        let user_password = &user.password.to_owned();

                                        match crud_bd::crud::user::update_user_username(
                                            app.pg_conn(),
                                            *user_id,
                                            user_name,
                                        ) {
                                            Ok(_) => app.set_prompt_message(Some(Ok(
                                                "User created".to_string(),
                                            ))),

                                            Err(err) => {
                                                app.fetch_users(n_rows);
                                                app.set_prompt_message(Some(Err(
                                                    std::io::Error::new(
                                                        std::io::ErrorKind::Other,
                                                        format!(
                                                            "Failed to create user username. {:?}",
                                                            err.to_string()
                                                        ),
                                                    ),
                                                )));
                                                return;
                                            }
                                        }

                                        match crud_bd::crud::user::update_user_password(
                                            app.pg_conn(),
                                            *user_id,
                                            user_password.as_str(),
                                        ) {
                                            Ok(_) => app.set_prompt_message(Some(Ok(
                                                "User updated".to_string(),
                                            ))),

                                            Err(err) => {
                                                app.fetch_users(n_rows);

                                                app.set_prompt_message(Some(Err(
                                                    std::io::Error::new(
                                                        std::io::ErrorKind::Other,
                                                        format!(
                                                            "Failed to update user password. {:?}",
                                                            err,
                                                        ),
                                                    ),
                                                )));
                                                return;
                                            }
                                        }
                                    }

                                    app.toggle_cursor_mode();
                                    app.set_focus_on(Some(AdminFocusOn::Line(row, 1)));
                                }

                                KeyCode::Char(c) => {
                                    if let Some(user) = app.users_mut().get_mut(row) {
                                        match col {
                                            1 => {
                                                user.username.push(c);
                                            }

                                            2 => {
                                                user.password.push(c);
                                            }

                                            _ => {}
                                        }
                                    }
                                }

                                KeyCode::Backspace => {
                                    if let Some(user) = app.users_mut().get_mut(row) {
                                        match col {
                                            1 => {
                                                user.username.pop();
                                            }

                                            2 => {
                                                user.password.pop();
                                            }

                                            _ => (),
                                        }
                                    }
                                }

                                KeyCode::Left => {
                                    if col > 1 {
                                        app.set_focus_on(Some(AdminFocusOn::Line(row, col - 1)));
                                    }
                                }

                                KeyCode::Right => {
                                    if col < 2 {
                                        app.set_focus_on(Some(AdminFocusOn::Line(row, col + 1)));
                                    }
                                }

                                _ => (),
                            }
                        }
                    }
                }

                AdminCursorMode::Edit('c') => {
                    if let Some(AdminFocusOn::Line(row, col)) = app.focus_on().clone() {
                        if let Some(Event::Key(key)) = current_event {
                            match key.code {
                                KeyCode::Enter => {
                                    let username = app.new_user().username.clone();
                                    let password = app.new_user().password.clone();

                                    match crud_bd::crud::user::create_user(
                                        app.pg_conn(),
                                        username.as_str(),
                                        password.as_str(),
                                    ) {
                                        Ok(_) => {
                                            app.set_prompt_message(Some(Ok(
                                                "User created".to_string()
                                            )));
                                        }

                                        Err(err) => {
                                            app.set_prompt_message(Some(Err(std::io::Error::new(
                                                std::io::ErrorKind::Other,
                                                format!("Failed to create user. {:?}", err),
                                            ))))
                                        }
                                    }

                                    app.toggle_cursor_mode();
                                    app.set_focus_on(Some(AdminFocusOn::Line(0, 1)));
                                    app.fetch_users(n_rows);
                                    app.new_user_mut().username.clear();
                                    app.new_user_mut().password.clear();
                                }

                                KeyCode::Char(c) => match col {
                                    1 => {
                                        app.new_user_mut().username.push(c);
                                    }

                                    2 => {
                                        app.new_user_mut().password.push(c);
                                    }

                                    _ => {}
                                },

                                KeyCode::Backspace => match col {
                                    1 => {
                                        app.new_user_mut().username.pop();
                                    }

                                    2 => {
                                        app.new_user_mut().password.pop();
                                    }

                                    _ => (),
                                },

                                KeyCode::Left => {
                                    if col > 1 {
                                        app.set_focus_on(Some(AdminFocusOn::Line(row, col - 1)));
                                    }
                                }

                                KeyCode::Right => {
                                    if col < 2 {
                                        app.set_focus_on(Some(AdminFocusOn::Line(row, col + 1)));
                                    }
                                }

                                _ => (),
                            }
                        }
                    }
                }

                _ => {}
            }
        }
        Ok(())
    }
    fn handle_resize(&mut self, app: &mut Admin, _: (u16, u16)) -> Result<()> {
        if let AdminCursorMode::Edit('c') = app.state().cursor_mode() {
            return Ok(());
        }

        self.users = match app.database().fetch_users(
            self.available_rows as i64,
            self.db_cursor,
            self.filter.clone(),
        ) {
            Ok(users) => users,
            Err(err) => {
                app.state_mut()
                    .set_prompt_message(Some(Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to fetch user. {:?}", err.to_string()),
                    ))));

                self.db_cursor = 0;
                Vec::new()
            }
        };

        if self.users.is_empty() {
            app.state_mut().set_focus_on(None);
        } else {
            match app.state().focus_on() {
                Some(AdminFocusOn::Line(n, _)) => {
                    if *n >= self.users.len() {
                        app.state_mut()
                            .set_focus_on(Some(AdminFocusOn::Line(self.users.len() - 1, 1)));
                    } else if self.users.len() <= self.chunks[1].height.saturating_sub(1) as usize {
                        self.db_cursor = (self.db_cursor as u64).saturating_sub(
                            1 + (self.chunks[1].height.saturating_sub(1) as usize
                                - self.users.len()) as u64,
                        ) as i64;
                    }
                }
                _ => {
                    app.state_mut().set_focus_on(Some(AdminFocusOn::Line(0, 1)));
                }
            }
        }

        Ok(())
    }
    fn cleanup(&mut self) -> Result<()> {
        // cleanup users
        // cleanup new user form
        // cleanup pagination
        Ok(())
    }
}

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
