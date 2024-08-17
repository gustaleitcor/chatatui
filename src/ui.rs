use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Color, Style},
    text::Text,
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
            if let Some(Event::Key(key)) = current_event {
                match &app.cursor_mode() {
                    CursorMode::Normal => match key.code {
                        KeyCode::Char('1') => app.set_current_screen(Login),
                        KeyCode::Char('q') => app.set_current_screen(Exit),
                        _ => {}
                    },
                    CursorMode::Insert => match key.code {
                        KeyCode::Enter => {
                            app.set_username("".to_string());
                            app.set_password("".to_string());
                            // app.set_current_screen(Chat);
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

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .flex(Flex::Center)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Max(2),
                        Constraint::Length(2),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            f.render_widget(
                Paragraph::new(Text::raw("Register").centered())
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::ALL)),
                chunks[0],
            );

            f.render_widget(
                Paragraph::new(Text::raw("Username:").centered())
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[1],
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
                chunks[2],
            );

            f.render_widget(
                Paragraph::new(Text::raw("Password:").alignment(Alignment::Center))
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[3],
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
                chunks[4],
            );

            f.render_widget(
                Paragraph::new(
                    Text::raw("ERROR: Username already exists")
                        .style(Style::default().fg(Color::Red)),
                )
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[5],
            );

            f.render_widget(
                Paragraph::new("Press '2' to switch to Register | Press 'Enter' to submit")
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .title(app.cursor_mode().as_str()),
                    ),
                chunks[6],
            );
        }
        Login => {
            if let Some(Event::Key(key)) = current_event {
                match &app.cursor_mode() {
                    CursorMode::Normal => match key.code {
                        KeyCode::Char('2') => app.set_current_screen(Register),
                        KeyCode::Char('q') => app.set_current_screen(Exit),
                        _ => {}
                    },
                    CursorMode::Insert => match key.code {
                        KeyCode::Enter => {
                            app.set_username("".to_string());
                            app.set_password("".to_string());
                            // app.set_current_screen(Chat);
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

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .flex(Flex::Center)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Max(2),
                        Constraint::Length(2),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            f.render_widget(
                Paragraph::new(Text::raw("Login").centered())
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::ALL)),
                chunks[0],
            );

            f.render_widget(
                Paragraph::new(Text::raw("Username:").centered())
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[1],
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
                chunks[2],
            );

            f.render_widget(
                Paragraph::new(Text::raw("Password:").alignment(Alignment::Center))
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[3],
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
                chunks[4],
            );

            // render message of error if any on chunk 5 put a dummy message for now

            f.render_widget(
                Paragraph::new(
                    Text::raw("ERROR: Invalid Username or Password")
                        .style(Style::default().fg(Color::Red)),
                )
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[5],
            );

            f.render_widget(
                Paragraph::new("Press '2' to switch to Register | Press 'Enter' to submit")
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .title(app.cursor_mode().as_str()),
                    ),
                chunks[6],
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
