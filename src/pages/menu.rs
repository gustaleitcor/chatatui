use std::{
    io::{Result, Stdout},
    rc::Rc,
};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    prelude::CrosstermBackend,
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

use crate::admin::{Admin, AdminCurrentScreen, AdminFocusOn};

use super::page::Page;

pub struct Menu {
    pub chunks: Rc<[Rect]>,
}

impl Page<CrosstermBackend<Stdout>> for Menu {
    fn render(&self, frame: &mut Frame, app_state: &mut Admin) -> Result<()> {
        frame.render_widget(
            Paragraph::new("Menu")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)),
            self.chunks[0],
        );

        frame.render_stateful_widget(
            List::new(["Users", "Messages", "Chats"])
                .scroll_padding(3)
                .highlight_symbol(" >> ")
                .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
            self.chunks[1],
            &mut ListState::default().with_selected(
                if let Some(AdminFocusOn::Line(n, _)) = app_state.focus_on() {
                    Some(*n)
                } else {
                    app_state.set_focus_on(Some(AdminFocusOn::Line(0, 1)));
                    Some(0)
                },
            ),
        );

        frame.render_widget(
            Paragraph::new("Press 'q' to exit")
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::TOP)
                        .title(app_state.cursor_mode().as_str()),
                ),
            self.chunks[2],
        );

        Ok(())
    }

    fn handle_input(&mut self, key: &KeyEvent, app_state: &mut Admin) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => {
                app_state.set_current_screen(AdminCurrentScreen::Exit);
            }

            KeyCode::Up => {
                if let Some(AdminFocusOn::Line(n, _)) = app_state.focus_on() {
                    if *n != 0 {
                        app_state.set_focus_on(Some(AdminFocusOn::Line((n - 1) % 3, 1)));
                    } else {
                        app_state.set_focus_on(Some(AdminFocusOn::Line(2, 1)));
                    }
                }
            }

            KeyCode::Down => {
                if let Some(AdminFocusOn::Line(n, _)) = app_state.focus_on() {
                    app_state.set_focus_on(Some(AdminFocusOn::Line((n + 1) % 3, 1)));
                }
            }

            KeyCode::Enter => {
                if let Some(AdminFocusOn::Line(n, _)) = app_state.focus_on().clone() {
                    app_state.set_prompt_message(None);
                    match n {
                        0 => {
                            app_state.set_current_screen(AdminCurrentScreen::Users);
                        }
                        1 => {
                            app_state.set_current_screen(AdminCurrentScreen::Messages);
                        }
                        2 => {
                            app_state.set_current_screen(AdminCurrentScreen::Chats);
                        }
                        _ => {}
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn setup(&mut self, frame: &mut Frame) -> Result<()> {
        self.chunks = Layout::default()
            .direction(Direction::Vertical)
            .flex(Flex::Center)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Fill(1),
                    Constraint::Length(2),
                ]
                .as_ref(),
            )
            .split(frame.size());

        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn handle_resize(&mut self, _: (u16, u16)) -> Result<()> {
        Ok(())
    }
}
