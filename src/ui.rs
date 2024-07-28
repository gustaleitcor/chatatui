use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, CurrentScreen::*};

pub fn ui(f: &mut Frame, app: &mut App) {
    let current_event = app.get_current_event();

    match *app.current_screen() {
        Register => {
            // handles event
            if let Some(Event::Key(key)) = current_event {
                if key.code == KeyCode::Tab {
                    app.set_current_screen(Login);
                    return;
                }

                if key.code == KeyCode::Esc {
                    app.set_current_screen(Exit);
                    return;
                }
            }

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
                if key.code == KeyCode::Tab {
                    app.set_current_screen(Register);
                    return;
                }

                if key.code == KeyCode::Esc {
                    app.set_current_screen(Exit);
                    return;
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
