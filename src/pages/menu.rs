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

use crate::{
    app::{App, FocusOn},
    state::State,
};

use super::page::Page;

pub struct Menu {
    pub chunks: Rc<[Rect]>,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            chunks: Rc::new([]),
        }
    }
}

impl Page<CrosstermBackend<Stdout>> for Menu {
    fn render(&self, frame: &mut Frame, state: &mut State) -> Result<()> {
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
                if let Some(FocusOn::Line(n, _)) = state.focus_on() {
                    Some(*n)
                } else {
                    state.set_focus_on(Some(FocusOn::Line(0, 1)));
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
                        .title(state.cursor_mode().as_str()),
                ),
            self.chunks[2],
        );

        Ok(())
    }

    fn handle_input(&mut self, app: &mut App, key: &KeyEvent) -> Result<()> {
        let state = app.state_mut();
        match key.code {
            KeyCode::Char('q') => state.goto_exit(),

            KeyCode::Up => {
                if let Some(FocusOn::Line(n, _)) = state.focus_on() {
                    if *n != 0 {
                        state.set_focus_on(Some(FocusOn::Line((n - 1) % 3, 1)));
                    } else {
                        state.set_focus_on(Some(FocusOn::Line(2, 1)));
                    }
                }
            }

            KeyCode::Down => {
                if let Some(FocusOn::Line(n, _)) = state.focus_on() {
                    state.set_focus_on(Some(FocusOn::Line((n + 1) % 3, 1)));
                }
            }

            KeyCode::Enter => {
                if let Some(FocusOn::Line(n, _)) = state.focus_on().clone() {
                    state.set_prompt_message(None);
                    match n {
                        0 => state.goto_users(),
                        1 => state.goto_messages(),
                        2 => state.goto_chats(),
                        _ => {}
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn layout(&mut self, frame: &mut Frame) -> Result<()> {
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

    fn handle_resize(&mut self, _: &mut App, _: (u16, u16)) -> Result<()> {
        Ok(())
    }

    fn setup(&mut self, app: &mut App) -> Result<()> {
        app.state_mut().set_focus_on(Some(FocusOn::Line(0, 1)));
        Ok(())
    }
}
