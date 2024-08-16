use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
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

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ])
                .split(f.size());

            let title_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());

            let title = Paragraph::new(Text::styled(
                "Create New Json",
                Style::default().fg(Color::Green),
            ))
            .block(title_block);

            // renders widget
            f.render_widget(
                Paragraph::new("Register")
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Alignment::Center),
                f.size(),
            );
        }
        Login => {
            if let Some(Event::Key(key)) = current_event {
                match key.code {
                    KeyCode::Tab => {
                        app.set_current_screen(Register);
                    }

                    KeyCode::Esc => {
                        app.toggle_cursor_mode();
                    }

                    _ => {}
                }
            }

            f.render_widget(
                Paragraph::new("Login")
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Alignment::Center),
                f.size(),
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
        Exit => {
            return;
        }
    }
}
