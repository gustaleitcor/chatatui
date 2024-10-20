use std::{
    io::{Result, Stdout},
    rc::Rc,
};

use crud_bd::crud::message::Message;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Color, CrosstermBackend},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

use std::cmp::min;

use crate::{
    app::{App, FocusOn},
    state::State,
};

use crate::page::Page;

pub struct Chat {
    pub chunks: Rc<[Rect]>,
    pub messages: Vec<Message>,
    pub message: String,
    pub chat_id: Option<i32>,
    pub available_rows: i64,
}

impl Chat {
    #[allow(dead_code)]
    pub fn new() -> Chat {
        Chat {
            chunks: Rc::new([]),
            messages: Vec::new(),
            message: String::new(),
            available_rows: 0,
            chat_id: None,
        }
    }
}

impl Chat {
    pub fn set_chat_id(&mut self, chat_id: i32) {
        self.chat_id = Some(chat_id);
    }
}

impl Page<CrosstermBackend<Stdout>> for Chat {
    fn render(&self, frame: &mut Frame, state: &mut State) -> Result<()> {
        frame.render_widget(
            Paragraph::new("Chat")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::TOP)),
            self.chunks[0],
        );

        frame.render_stateful_widget(
            List::new(self.messages.iter().map(|m| Text::raw(m.content.clone())))
                .scroll_padding(self.messages.len() / 2)
                .highlight_symbol(" >> "),
            self.chunks[1],
            &mut ListState::default()
                .with_selected(if let Some(FocusOn::Line(n, _)) = state.focus_on() {
                    if *n == 0 {
                        None
                    } else {
                        Some(self.messages.len().saturating_sub(*n))
                    }
                } else {
                    None
                })
                .with_offset(if let Some(FocusOn::Line(n, _)) = state.focus_on() {
                    self.messages
                        .len()
                        .saturating_sub(self.available_rows as usize)
                } else {
                    0
                }),
        );

        frame.render_widget(
            Paragraph::new(self.message.as_str())
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::LEFT)
                        .border_style(Style::default().fg(Color::Yellow)),
                ),
            self.chunks[2],
        );

        frame.render_widget(
            Paragraph::new("Press 'q' to exit")
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::TOP)
                        .title(state.cursor_mode().as_str()),
                ),
            self.chunks[3],
        );
        Ok(())
    }

    fn handle_input(&mut self, app: &mut App, key: &KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => app.state_mut().goto_client_chats(),
            KeyCode::Tab => app.state_mut().set_focus_on(Some(FocusOn::Line(0, 0))),
            KeyCode::Up => match app.state().focus_on().clone() {
                Some(FocusOn::Line(0, _)) => app
                    .state_mut()
                    .set_focus_on(Some(FocusOn::Line(min(1, self.messages.len()), 0))),
                Some(FocusOn::Line(n, _)) => {
                    if n < self.messages.len() {
                        app.state_mut().set_focus_on(Some(FocusOn::Line(n + 1, 0)))
                    }
                }
                _ => {}
            },

            KeyCode::Down => {
                if let Some(FocusOn::Line(n, _)) = app.state().focus_on().clone() {
                    if n <= 1 {
                        app.state_mut().set_focus_on(Some(FocusOn::Line(0, 0)))
                    } else {
                        app.state_mut().set_focus_on(Some(FocusOn::Line(n - 1, 0)));
                    }
                }
            }

            KeyCode::Enter => {
                let user_id = app.state().user().id;

                let chat_id = match self.chat_id {
                    Some(id) => id,
                    None => {
                        app.state_mut()
                            .set_prompt_message(Some(Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "Chat id is not set",
                            ))));
                        app.state_mut().goto_login();
                        return Ok(());
                    }
                };

                let msg =
                    match app
                        .database()
                        .create_message(user_id, chat_id, self.message.as_str())
                    {
                        Ok(msg) => msg,
                        Err(err) => {
                            app.state_mut()
                                .set_prompt_message(Some(Err(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    format!("Failed to fetch chats. {:?}", err.to_string()),
                                ))));
                            return Ok(());
                        }
                    };

                self.messages.push(Message {
                    id: msg.id,
                    participant_id: msg.participant_id,
                    content: msg.content,
                    date: msg.date,
                });

                self.message.clear();
            }

            KeyCode::Char(c) => match app.state().focus_on() {
                Some(FocusOn::Line(0, _)) => self.message.push(c),
                _ => app.state_mut().set_focus_on(Some(FocusOn::Line(0, 0))),
            },

            KeyCode::Backspace => match app.state().focus_on() {
                Some(FocusOn::Line(0, _)) => {
                    self.message.pop();
                }
                _ => app.state_mut().set_focus_on(Some(FocusOn::Line(0, 0))),
            },

            _ => {}
        }

        Ok(())
    }

    fn layout(&mut self, frame: &mut Frame) -> Result<()> {
        self.chunks = Layout::default()
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
            .split(frame.area());

        self.available_rows = self.chunks[1].height.saturating_sub(1) as i64;
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn handle_resize(&mut self, _: &mut App, _: (u16, u16)) -> Result<()> {
        Ok(())
    }

    fn setup(&mut self, app: &mut App) -> Result<()> {
        self.messages = match app.database().get_chat_messages(self.chat_id.unwrap()) {
            Ok(messages) => messages,
            Err(err) => {
                app.state_mut()
                    .set_prompt_message(Some(Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to fetch messages. {:?}", err.to_string()),
                    ))));
                return Ok(());
            }
        };
        Ok(())
    }
}
