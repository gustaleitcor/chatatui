use std::{cmp::min_by, ops::Rem};

use crud_bd::crud::user::User;
use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::{Alignment, Constraint, Direction, Layout, Rows},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Cell, List, ListState, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::admin::{Admin, AdminCurrentScreen, AdminCursorMode, AdminFocusOn};

pub fn ui_admin(f: &mut Frame, app: &mut Admin) {
    let current_event = app.take_current_event();

    if let Some(Event::Key(key)) = current_event {
        if key.code == KeyCode::Esc {
            app.toggle_cursor_mode();
        }
    }

    // renders the current screen
    match *app.current_screen() {
        AdminCurrentScreen::Menu => {
            if let Some(Event::Key(key)) = current_event {
                match key.code {
                    KeyCode::Char('q') => {
                        app.set_current_screen(AdminCurrentScreen::Exit);
                    }

                    KeyCode::Down => {
                        if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
                            app.set_focus_on(Some(AdminFocusOn::Line((n + 1) % 3, 0)));
                        }
                    }

                    KeyCode::Up => {
                        if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
                            if *n != 0 {
                                app.set_focus_on(Some(AdminFocusOn::Line((n - 1) % 3, 0)));
                            } else {
                                app.set_focus_on(Some(AdminFocusOn::Line(2, 0)));
                            }
                        }
                    }

                    KeyCode::Enter => {
                        if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
                            match n {
                                0 => {
                                    app.set_db_cursor(0);
                                    app.fetch_users(f.size().height.saturating_sub(7) as i64); // MAGIC NUMBER 7
                                    app.set_current_screen(AdminCurrentScreen::Users);
                                }
                                1 => {
                                    app.set_current_screen(AdminCurrentScreen::Messages);
                                }
                                2 => {
                                    app.set_current_screen(AdminCurrentScreen::Chats);
                                }
                                _ => {}
                            }
                        }
                    }

                    _ => {}
                }
            }

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(2),
                        Constraint::Fill(1),
                        Constraint::Length(2),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            f.render_widget(
                Paragraph::new("Menu")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)),
                chunks[0],
            );

            f.render_stateful_widget(
                List::new(["Users", "Messages", "Chats"])
                    .scroll_padding(3)
                    .highlight_symbol(" >> ")
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[1],
                &mut ListState::default().with_selected(
                    if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
                        Some(*n)
                    } else {
                        app.set_focus_on(Some(AdminFocusOn::Line(0, 0)));
                        Some(0)
                    },
                ),
            );

            f.render_widget(
                Paragraph::new("Press 'q' to exit")
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .title(app.cursor_mode().as_str()),
                    ),
                chunks[2],
            );
        }

        AdminCurrentScreen::Users => {
            let chunks = Layout::default()
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
                .split(f.size());

            if let Some(Event::Resize(_, _)) = current_event {
                app.fetch_users(chunks[1].height.saturating_sub(1) as i64);

                if app.users().is_empty() {
                    app.set_focus_on(None);
                } else {
                    match app.focus_on() {
                        Some(AdminFocusOn::Line(n, _)) => {
                            if *n >= app.users().len() {
                                app.set_focus_on(Some(AdminFocusOn::Line(
                                    app.users().len() - 1,
                                    0,
                                )));
                            }
                        }
                        _ => {
                            app.set_focus_on(Some(AdminFocusOn::Line(0, 0)));
                        }
                    }
                }
            }

            if let Some(Event::Key(key)) = current_event {
                match app.cursor_mode() {
                    AdminCursorMode::View => match key.code {
                        KeyCode::Char('q') => {
                            app.set_current_screen(AdminCurrentScreen::Menu);
                        }
                        KeyCode::Down => {
                            if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
                                if !app.users().is_empty() {
                                    if *n < app.users().len() - 1 {
                                        app.set_focus_on(Some(AdminFocusOn::Line(n + 1, 0)));
                                    } else {
                                        app.next_users_page(
                                            chunks[1].height.saturating_sub(1) as i64
                                        );
                                        app.set_focus_on(Some(AdminFocusOn::Line(0, 0)));
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
                                        app.set_focus_on(Some(AdminFocusOn::Line(n - 1, 0)));
                                    } else {
                                        app.prev_users_page(
                                            chunks[1].height.saturating_sub(1) as i64
                                        );
                                        app.set_focus_on(Some(AdminFocusOn::Line(
                                            app.users().len() - 1,
                                            0,
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
                                if !app.users().is_empty() {
                                    let user = app.users().get(*n).unwrap();
                                    let user_id = user.id.to_owned();
                                    match crud_bd::crud::user::delete_user(app.pg_conn(), user_id) {
                                        Ok(_) => {
                                            app.fetch_users(
                                                chunks[1].height.saturating_sub(1) as i64
                                            );
                                            app.set_focus_on(Some(AdminFocusOn::Line(0, 0)));
                                        }

                                        Err(_) => {
                                            app.set_error("Failed to delete user".to_string());
                                        }
                                    }
                                }
                            }
                        }

                        KeyCode::Char('u') => {
                            if !app.users().is_empty() {
                                if let Some(AdminFocusOn::Line(row, _)) = app.focus_on() {
                                    if let Some(user) = app.users().get(*row) {
                                        app.set_user(User {
                                            id: user.id.to_owned(),
                                            username: user.username.to_owned(),
                                            password: user.password.to_owned(),
                                        });
                                        app.set_cursor_mode(AdminCursorMode::Edit('u'));
                                    }
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
                                                Ok(_) => {
                                                    app.set_error("User updated".to_string());
                                                }

                                                Err(_) => {
                                                    app.set_error(
                                                        "Failed to update user".to_string(),
                                                    );
                                                }
                                            }

                                            match crud_bd::crud::user::update_user_password(
                                                app.pg_conn(),
                                                *user_id,
                                                user_password.as_str(),
                                            ) {
                                                Ok(_) => {
                                                    app.set_error("User updated".to_string());
                                                }

                                                Err(_) => {
                                                    app.set_error(
                                                        "Failed to update user".to_string(),
                                                    );
                                                }
                                            }
                                        }

                                        app.toggle_cursor_mode();
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
                                            app.set_focus_on(Some(AdminFocusOn::Line(
                                                row,
                                                col - 1,
                                            )));
                                        }
                                    }

                                    KeyCode::Right => {
                                        if col < 2 {
                                            app.set_focus_on(Some(AdminFocusOn::Line(
                                                row,
                                                col + 1,
                                            )));
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

            let line = match app.focus_on() {
                Some(AdminFocusOn::Line(n, _)) => *n as i32,
                _ => -1,
            };

            f.render_widget(
                Paragraph::new("Users").alignment(Alignment::Center).block(
                    Block::default()
                        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                        .title(format!(
                            "db_cursor: {} | Page: {} | Userslen: {} | Line: {} | TableHeight: {}",
                            app.db_cursor(),
                            app.db_cursor()
                                .checked_div(chunks[1].as_size().height.saturating_sub(1) as i64)
                                .unwrap_or(0),
                            app.users().len(),
                            line,
                            chunks[1].as_size().height
                        )),
                ),
                chunks[0],
            );

            let header = ["Id", "Name", "Pwd"]
                .into_iter()
                .map(Cell::from)
                .collect::<Row>()
                .style(Style::default().fg(Color::Yellow))
                .height(1);

            let rows = app.users().iter().enumerate().map(|(i, data)| {
                if let AdminCursorMode::Edit('u') = app.cursor_mode() {
                    if let Some(AdminFocusOn::Line(row, col)) = app.focus_on() {
                        if i == *row {
                            let mut cells = vec![];
                            for (j, cell) in [
                                data.id.to_string(),
                                data.username.to_owned(),
                                data.password.to_owned(),
                            ]
                            .iter()
                            .enumerate()
                            {
                                if j == *col {
                                    cells.push(
                                        Cell::from(Text::from(cell.to_owned()))
                                            .style(Style::default().bg(Color::LightBlue)),
                                    );
                                } else {
                                    cells.push(Cell::from(Text::from(cell.to_owned())));
                                }
                            }
                            return Row::new(cells);
                        }
                    }
                }

                let id = Cell::from(Text::from(format!("{}", data.id)));
                let username = Cell::from(Text::from(data.username.to_owned()));
                let password = Cell::from(Text::from(data.password.to_owned()));

                [id, username, password].into_iter().collect::<Row>()
            });

            f.render_stateful_widget(
                Table::new(
                    rows,
                    [
                        // + 1 is for padding.
                        Constraint::Length(5),
                        Constraint::Max(25),
                        Constraint::Max(25),
                    ],
                )
                .header(header)
                .highlight_symbol(" >> ")
                .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[1],
                &mut TableState::new().with_selected(
                    if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
                        Some(*n)
                    } else {
                        Some(0)
                    },
                ),
            );

            if let Some(msg) = app.error() {
                f.render_widget(
                    Paragraph::new(msg.to_owned())
                        .alignment(Alignment::Center)
                        .style(Style::default().fg(Color::Red))
                        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
                    chunks[2],
                );
            } else {
                f.render_widget(
                    Block::default().borders(Borders::LEFT | Borders::RIGHT),
                    chunks[2],
                );
            }

            match app.cursor_mode() {
                AdminCursorMode::View => {
                    f.render_widget(
                        Paragraph::new("Press 'f' to filter | Press 'q' to goto menu")
                            .alignment(Alignment::Center)
                            .block(
                                Block::default()
                                    .borders(Borders::TOP)
                                    .title(app.cursor_mode().as_str()),
                            ),
                        chunks[3],
                    );
                }

                AdminCursorMode::Edit(_) => {
                    f.render_widget(
                        Paragraph::new("Press 'd' to delete | Press 'e' to edit")
                            .alignment(Alignment::Center)
                            .block(
                                Block::default()
                                    .borders(Borders::TOP)
                                    .title(app.cursor_mode().as_str()),
                            ),
                        chunks[3],
                    );
                }
            }
        }

        AdminCurrentScreen::Messages => todo!(""),
        AdminCurrentScreen::Chats => todo!(""),
        AdminCurrentScreen::Exit => {}
    }
}
