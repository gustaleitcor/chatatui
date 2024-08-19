use crud_bd::crud::user::{self, username_exists};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::{App, CurrentScreen::*, CursorMode, FocusOn};

pub fn ui(f: &mut Frame, app: &mut App) {
    let mut current_event = app.take_current_event();

    if let Some(Event::Key(key)) = current_event {
        match key.code {
            KeyCode::Esc => {
                app.set_cursor_mode(CursorMode::Normal);
            }
            KeyCode::Char('a') => {
                if let CursorMode::Normal = app.cursor_mode() {
                    current_event = None;
                }
                app.set_cursor_mode(CursorMode::Insert);
            }

            _ => {}
        }
    }

    // renders the current screen
    match *app.current_screen() {
        Register => {
            if let Some(Event::Key(key)) = current_event {
                match &app.cursor_mode() {
                    CursorMode::Normal => match key.code {
                        KeyCode::Char('1') => {
                            app.set_current_screen(Login);
                            app.set_error("".to_string());
                        }
                        KeyCode::Char('q') => app.set_current_screen(Exit),
                        _ => {}
                    },
                    CursorMode::Insert => match key.code {
                        KeyCode::Enter => {
                            let username = app.username().to_string().clone();
                            let password = app.password().to_string().clone();

                            if username.is_empty() || password.is_empty() {
                                app.set_error("Username or password cannot be empty".to_string());
                            } else if username_exists(app.pgConn(), &username) {
                                app.set_error("Username already exists".to_string());
                            } else {
                                user::create_user(app.pgConn(), &username, &password);
                                app.set_error("".to_string());
                                app.set_current_screen(Login);
                            }
                        }

                        KeyCode::Char(c) => match app.focus_on() {
                            Some(FocusOn::Username) => app.push_username(c),
                            Some(FocusOn::Password) => app.push_password(c),
                            _ => app.set_focus_on(Some(FocusOn::Username)),
                        },

                        KeyCode::Backspace => match app.focus_on() {
                            Some(FocusOn::Username) => app.pop_username(),
                            Some(FocusOn::Password) => app.pop_password(),
                            _ => app.set_focus_on(Some(FocusOn::Username)),
                        },

                        KeyCode::Tab => match app.focus_on() {
                            Some(FocusOn::Username) => app.set_focus_on(Some(FocusOn::Password)),
                            Some(FocusOn::Password) => app.set_focus_on(Some(FocusOn::Username)),
                            _ => app.set_focus_on(Some(FocusOn::Username)),
                        },

                        _ => {}
                    },
                }
            }

            let chunks: [Rect; 6] = Layout::default()
                .direction(Direction::Vertical)
                .flex(Flex::Center)
                .constraints(
                    [
                        Constraint::Length(2),
                        Constraint::Max(3),
                        Constraint::Length(1),
                        Constraint::Max(3),
                        Constraint::Length(1),
                        Constraint::Length(2),
                    ]
                    .as_ref(),
                )
                .areas(f.size());

            f.render_widget(
                Paragraph::new(Text::raw("Username:").centered())
                    .alignment(Alignment::Center)
                    .block(
                        Block::new()
                            .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                            .title("Register"),
                    ),
                chunks[0],
            );

            f.render_widget(
                Paragraph::new(Text::raw(app.username()).centered())
                    .centered()
                    .block(
                        Block::new()
                            .borders(if let Some(FocusOn::Username) = app.focus_on() {
                                Borders::ALL
                            } else {
                                Borders::LEFT | Borders::RIGHT
                            })
                            .border_style(Style::default().fg(
                                if let Some(FocusOn::Username) = app.focus_on() {
                                    Color::Yellow
                                } else {
                                    Color::Reset
                                },
                            )),
                    ),
                chunks[1],
            );

            f.render_widget(
                Paragraph::new(Text::raw("Password:").alignment(Alignment::Center))
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[2],
            );

            f.render_widget(
                Paragraph::new(Text::raw(app.password()).centered())
                    .alignment(Alignment::Center)
                    .block(
                        Block::new()
                            .borders(if let Some(FocusOn::Password) = app.focus_on() {
                                Borders::ALL
                            } else {
                                Borders::LEFT | Borders::RIGHT
                            })
                            .border_style(Style::default().fg(
                                if let Some(FocusOn::Password) = app.focus_on() {
                                    Color::Yellow
                                } else {
                                    Color::Reset
                                },
                            )),
                    ),
                chunks[3],
            );

            f.render_widget(
                Paragraph::new(Text::raw(app.error()).style(Style::default().fg(Color::Red)))
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[4],
            );

            match app.cursor_mode() {
                CursorMode::Normal => f.render_widget(
                    Paragraph::new("Press '1' to switch to Login | Press 'q' to exit")
                        .alignment(Alignment::Center)
                        .block(
                            Block::default()
                                .borders(Borders::TOP)
                                .title(app.cursor_mode().as_str()),
                        ),
                    chunks[5],
                ),
                CursorMode::Insert => f.render_widget(
                    Paragraph::new("Press 'Enter' to submit")
                        .alignment(Alignment::Center)
                        .block(
                            Block::default()
                                .borders(Borders::TOP)
                                .title(app.cursor_mode().as_str()),
                        ),
                    chunks[5],
                ),
            }
        }
        Login => {
            if let Some(Event::Key(key)) = current_event {
                match &app.cursor_mode() {
                    CursorMode::Normal => match key.code {
                        KeyCode::Char('2') => {
                            app.set_current_screen(Register);
                            app.set_error("".to_string())
                        }
                        KeyCode::Char('q') => app.set_current_screen(Exit),
                        _ => {}
                    },
                    CursorMode::Insert => match key.code {
                        KeyCode::Enter => {
                            let username = app.username().to_string().clone();
                            let response = user::get_user_by_username(app.pgConn(), &username);
                            match response {
                                Ok(user) => {
                                    if user.password == app.password()
                                        && user.username == app.username()
                                    {
                                        app.set_current_screen(Chat);
                                        app.set_error("".to_string());
                                    } else {
                                        app.set_error("Invalid password".to_string());
                                    }
                                }
                                Err(_) => app.set_error("Invalid username".to_string()),
                            }
                        }

                        KeyCode::Char(c) => match app.focus_on() {
                            Some(FocusOn::Username) => app.push_username(c),
                            Some(FocusOn::Password) => app.push_password(c),
                            _ => app.set_focus_on(Some(FocusOn::Username)),
                        },
                        KeyCode::Backspace => match app.focus_on() {
                            Some(FocusOn::Username) => app.pop_username(),
                            Some(FocusOn::Password) => app.pop_password(),
                            _ => app.set_focus_on(Some(FocusOn::Username)),
                        },

                        KeyCode::Tab => match app.focus_on() {
                            Some(FocusOn::Username) => app.set_focus_on(Some(FocusOn::Password)),
                            Some(FocusOn::Password) => app.set_focus_on(Some(FocusOn::Username)),
                            _ => app.set_focus_on(Some(FocusOn::Username)),
                        },

                        _ => {}
                    },
                }
            }

            let chunks: [Rect; 6] = Layout::default()
                .direction(Direction::Vertical)
                .flex(Flex::Center)
                .constraints(
                    [
                        Constraint::Length(2),
                        Constraint::Max(3),
                        Constraint::Length(1),
                        Constraint::Max(3),
                        Constraint::Length(1),
                        Constraint::Length(2),
                    ]
                    .as_ref(),
                )
                .areas(f.size());

            f.render_widget(
                Paragraph::new(Text::raw("Username:").centered())
                    .alignment(Alignment::Center)
                    .block(
                        Block::new()
                            .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                            .title("Login"),
                    ),
                chunks[0],
            );

            f.render_widget(
                Paragraph::new(Text::raw(app.username()).centered())
                    .centered()
                    .block(
                        Block::new()
                            .borders(if let Some(FocusOn::Username) = app.focus_on() {
                                Borders::ALL
                            } else {
                                Borders::LEFT | Borders::RIGHT
                            })
                            .border_style(Style::default().fg(
                                if let Some(FocusOn::Username) = app.focus_on() {
                                    Color::Yellow
                                } else {
                                    Color::Reset
                                },
                            )),
                    ),
                chunks[1],
            );

            f.render_widget(
                Paragraph::new(Text::raw("Password:").alignment(Alignment::Center))
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[2],
            );

            f.render_widget(
                Paragraph::new(
                    Text::raw(app.password().chars().map(|_| '*').collect::<String>()).centered(),
                )
                .alignment(Alignment::Center)
                .block(
                    Block::new()
                        .borders(if let Some(FocusOn::Password) = app.focus_on() {
                            Borders::ALL
                        } else {
                            Borders::LEFT | Borders::RIGHT
                        })
                        .border_style(Style::default().fg(
                            if let Some(FocusOn::Password) = app.focus_on() {
                                Color::Yellow
                            } else {
                                Color::Reset
                            },
                        )),
                ),
                chunks[3],
            );

            // render message of error if any on chunk 5 put a dummy message for now

            f.render_widget(
                Paragraph::new(Text::raw(app.error()).style(Style::default().fg(Color::Red)))
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[4],
            );

            match app.cursor_mode() {
                CursorMode::Normal => f.render_widget(
                    Paragraph::new("Press '2' to switch to Register | Press 'q' to exit")
                        .alignment(Alignment::Center)
                        .block(
                            Block::default()
                                .borders(Borders::TOP)
                                .title(app.cursor_mode().as_str()),
                        ),
                    chunks[5],
                ),
                CursorMode::Insert => f.render_widget(
                    Paragraph::new("Press 'Enter' to submit")
                        .alignment(Alignment::Center)
                        .block(
                            Block::default()
                                .borders(Borders::TOP)
                                .title(app.cursor_mode().as_str()),
                        ),
                    chunks[5],
                ),
            }
        }
        Chat => {
            if let Some(Event::Key(key)) = current_event {
                match &app.cursor_mode() {
                    CursorMode::Normal => match key.code {
                        KeyCode::Char('q') => app.set_current_screen(Exit),
                        KeyCode::Tab => app.set_current_screen(Chat),
                        _ => {}
                    },
                    CursorMode::Insert => match key.code {
                        KeyCode::Enter => {
                            app.push_message(app.message().to_string());
                            app.set_message("".to_string());
                        }

                        KeyCode::Char(c) => match app.focus_on() {
                            Some(FocusOn::Input) => app.push_message_char(c),
                            _ => app.set_focus_on(Some(FocusOn::Input)),
                        },

                        KeyCode::Backspace => match app.focus_on() {
                            Some(FocusOn::Input) => app.pop_message(),
                            _ => app.set_focus_on(Some(FocusOn::Input)),
                        },

                        KeyCode::Tab => match app.focus_on() {
                            Some(FocusOn::Input) => app.set_focus_on(Some(FocusOn::Input)),
                            _ => app.set_focus_on(Some(FocusOn::Input)),
                        },

                        _ => {}
                    },
                }
            }

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(2),
                        Constraint::Fill(1),
                        Constraint::Length(1),
                        Constraint::Length(2),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            f.render_widget(
                Paragraph::new("Chat")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::TOP)),
                chunks[0],
            );

            f.render_stateful_widget(
                List::new(app.messages().to_vec()).scroll_padding(1),
                chunks[1],
                &mut ListState::default()
                    .with_selected(Some(app.messages().len()))
                    .with_offset(2),
            );

            f.render_widget(
                Paragraph::new(app.message().to_string())
                    .alignment(Alignment::Left)
                    .block(
                        Block::default()
                            .borders(Borders::LEFT)
                            .border_style(Style::default().fg(Color::Yellow)),
                    ),
                chunks[2],
            );

            f.render_widget(
                Paragraph::new("Press 'q' to exit")
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .title(app.cursor_mode().as_str()),
                    ),
                chunks[3],
            );
        }
        Exit => {}
    }
}
