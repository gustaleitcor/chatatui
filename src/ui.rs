use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, CurrentScreen::*, CursorMode, FocusOn};

pub fn ui(f: &mut Frame, app: &mut App) {
    let current_event = app.take_current_event();

    if let Some(Event::Key(key)) = current_event {
        match key.code {
            KeyCode::Esc => app.set_cursor_mode(CursorMode::Normal),
            KeyCode::Char('a') => app.set_cursor_mode(CursorMode::Insert),
            _ => {}
        }
    }

    // renders the current screen
    match *app.current_screen() {
        Register => {
            // handles event
            if let Some(Event::Key(key)) = current_event {
                if key.code == KeyCode::Char('1') {
                    app.set_current_screen(Login);
                }

                if key.code == KeyCode::Esc {
                    app.set_current_screen(Exit);
                }
            }

            // make a block with a border with tile "Register", inside has two fiels, username and password

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .flex(Flex::Center)
                .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
                .split(f.size());

            f.render_widget(
                Paragraph::new("Username")
                    .alignment(Alignment::Center)
                    .block(
                        Block::new()
                            .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                            .title("Register"),
                    ),
                chunks[0],
            );

            f.render_widget(
                Paragraph::new("Password")
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)),
                chunks[1],
            );
        }
        Login => {
            if let Some(Event::Key(key)) = current_event {
                match key.code {
                    KeyCode::Char('2') => {
                        app.set_username("".to_string());
                        app.set_password("".to_string());
                        app.set_current_screen(Register);
                        return;
                    }

                    _ => match &app.cursor_mode() {
                        CursorMode::Normal => {}
                        CursorMode::Insert => match key.code {
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
                                Some(FocusOn::Username) => {
                                    app.set_focus_on(Some(FocusOn::Password))
                                }
                                Some(FocusOn::Password) => {
                                    app.set_focus_on(Some(FocusOn::Username))
                                }
                                _ => app.set_focus_on(Some(FocusOn::Username)),
                            },

                            _ => {}
                        },
                    },
                }
            }

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .flex(Flex::Center)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Fill(1),
                        Constraint::Fill(1),
                        Constraint::Fill(1),
                        Constraint::Fill(1),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let title_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints([Constraint::Max(f.size().width / 2)])
                .split(chunks[0]);

            let login_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints([Constraint::Max(f.size().width / 2)])
                .split(chunks[1]);

            let login_input_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints([Constraint::Max(f.size().width / 2)])
                .split(chunks[2]);

            let password_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints([Constraint::Max(f.size().width / 2)])
                .split(chunks[3]);

            let password_input_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints([Constraint::Max(f.size().width / 2)])
                .split(chunks[4]);

            f.render_widget(
                Paragraph::new("Login").alignment(Alignment::Center).block(
                    Block::new()
                        .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                        .title(app.cursor_mode().as_str()),
                ),
                title_chunk[0],
            );

            f.render_widget(
                Paragraph::new("Username")
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
                login_chunk[0],
            );

            f.render_widget(
                Paragraph::new(app.username())
                    .alignment(Alignment::Center)
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
                login_input_chunk[0],
            );

            f.render_widget(
                Paragraph::new("Password")
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
                password_chunk[0],
            );

            f.render_widget(
                Paragraph::new(app.password())
                    .alignment(Alignment::Center)
                    .block(
                        Block::new()
                            .borders(if let Some(FocusOn::Password) = app.focus_on() {
                                Borders::ALL
                            } else {
                                Borders::LEFT | Borders::RIGHT | Borders::BOTTOM
                            })
                            .border_style(Style::default().fg(
                                if let Some(FocusOn::Password) = app.focus_on() {
                                    Color::Yellow
                                } else {
                                    Color::Reset
                                },
                            )),
                    ),
                password_input_chunk[0],
            );

            // render footer with the key options labeled

            f.render_widget(
                Paragraph::new("Press 'Tab' to switch to Register | Press 'Enter' to submit")
                    .alignment(Alignment::Center),
                chunks[5],
            );
        }
        Chat => {
            f.render_widget(
                Paragraph::new("Chat")
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Alignment::Center),
                f.size(),
            );
        }
        Exit => {}
    }
}
