use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, CurrentScreen::*, CursorMode};

pub fn ui(f: &mut Frame, app: &mut App) {
    let current_event = app.take_current_event();

    if let Some(Event::Key(key)) = current_event {
        match key.code {
            KeyCode::Esc => app.set_cursor_mode(CursorMode::Normal),
            KeyCode::Char('a') => app.set_cursor_mode(CursorMode::Insert),
            _ => {}
        }
    }

    match *app.current_screen() {
        Register => {
            // handles event
            if let Some(Event::Key(key)) = current_event {
                if key.code == KeyCode::Tab {
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
                    KeyCode::Tab => {
                        app.set_username("".to_string());
                        app.set_password("".to_string());
                        app.set_current_screen(Register);
                    }

                    KeyCode::Esc => {
                        app.toggle_cursor_mode();
                    }

                    _ => match &app.cursor_mode() {
                        CursorMode::Normal => {}
                        CursorMode::Insert => match key.code {
                            KeyCode::Char(c) => {
                                app.set_username(format!("{}{}", app.username(), c));
                            }
                            KeyCode::Backspace => {
                                app.set_username(
                                    app.username()
                                        .chars()
                                        .take(app.username().len() - 1)
                                        .collect(),
                                );
                            }
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
                        Constraint::Fill(1),
                        Constraint::Fill(1),
                        Constraint::Fill(1),
                        Constraint::Fill(1),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let login_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints([Constraint::Max(f.size().width / 2)])
                .split(chunks[0]);

            let login_input_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints([Constraint::Max(f.size().width / 2)])
                .split(chunks[1]);

            let password_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints([Constraint::Max(f.size().width / 2)])
                .split(chunks[2]);

            let password_input_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints([Constraint::Max(f.size().width / 2)])
                .split(chunks[3]);

            f.render_widget(
                Paragraph::new("Username")
                    .alignment(Alignment::Center)
                    .block(
                        Block::new()
                            .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                            .title("Login"),
                    ),
                login_chunk[0],
            );

            f.render_widget(
                Paragraph::new(app.username())
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
                login_input_chunk[0],
            );

            f.render_widget(
                Paragraph::new("Password")
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
                password_chunk[0],
            );

            f.render_widget(
                Paragraph::new(app.username())
                    .alignment(Alignment::Center)
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)),
                password_input_chunk[0],
            );

            // render footer with the key options labeled

            f.render_widget(
                Paragraph::new("Press 'Tab' to switch to Register").alignment(Alignment::Center),
                chunks[4],
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
