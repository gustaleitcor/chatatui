use std::{
    io::{Result, Stdout},
    rc::Rc,
};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    prelude::Color,
    prelude::CrosstermBackend,
    style::Style,
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    app::{App, CursorMode, FocusOn},
    state::State,
};

use crate::page::Page;

pub struct Register {
    pub chunks: Rc<[Rect]>,
    pub login: String,
    pub password: String,
}

impl Register {
    pub fn new() -> Register {
        Register {
            chunks: Rc::new([]),
            login: String::new(),
            password: String::new(),
        }
    }
}

impl Page<CrosstermBackend<Stdout>> for Register {
    fn render(&self, frame: &mut Frame, state: &mut State) -> Result<()> {
        frame.render_widget(
            Paragraph::new(Text::raw("Username:").centered())
                .alignment(Alignment::Center)
                .block(
                    Block::new()
                        .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                        .title("Register"),
                ),
            self.chunks[0],
        );

        frame.render_widget(
            Paragraph::new(Text::raw(&self.login).centered())
                .centered()
                .block(
                    Block::new()
                        .borders(if let Some(FocusOn::Line(0, _)) = state.focus_on() {
                            Borders::ALL
                        } else {
                            Borders::LEFT | Borders::RIGHT
                        })
                        .border_style(Style::default().fg(
                            if let Some(FocusOn::Line(0, _)) = state.focus_on() {
                                Color::Yellow
                            } else {
                                Color::Reset
                            },
                        )),
                ),
            self.chunks[1],
        );

        frame.render_widget(
            Paragraph::new(Text::raw("Password:").alignment(Alignment::Center))
                .alignment(Alignment::Center)
                .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
            self.chunks[2],
        );

        frame.render_widget(
            Paragraph::new(Text::raw(&self.password).centered())
                .alignment(Alignment::Center)
                .block(
                    Block::new()
                        .borders(if let Some(FocusOn::Line(1, _)) = state.focus_on() {
                            Borders::ALL
                        } else {
                            Borders::LEFT | Borders::RIGHT
                        })
                        .border_style(Style::default().fg(
                            if let Some(FocusOn::Line(1, _)) = state.focus_on() {
                                Color::Yellow
                            } else {
                                Color::Reset
                            },
                        )),
                ),
            self.chunks[3],
        );

        if let Some(Ok(msg)) = state.prompt_message() {
            frame.render_widget(
                Paragraph::new(msg.to_owned())
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
                self.chunks[4],
            );
        } else if let Some(Err(msg)) = state.prompt_message() {
            frame.render_widget(
                Paragraph::new(msg.to_string())
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Red))
                    .block(
                        Block::default()
                            .borders(Borders::LEFT | Borders::RIGHT)
                            .style(Style::default().fg(Color::White)),
                    ),
                self.chunks[4],
            );
        } else {
            frame.render_widget(
                Block::default().borders(Borders::LEFT | Borders::RIGHT),
                self.chunks[4],
            );
        };

        match state.cursor_mode() {
            CursorMode::View(_) => frame.render_widget(
                Paragraph::new("Press '1' to switch to Login | Press 'q' to exit")
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .title(state.cursor_mode().as_str()),
                    ),
                self.chunks[5],
            ),
            CursorMode::Edit(_) => frame.render_widget(
                Paragraph::new("Press 'Enter' to submit")
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .title(state.cursor_mode().as_str()),
                    ),
                self.chunks[5],
            ),
        }
        Ok(())
    }

    fn handle_input(&mut self, app: &mut App, key: &KeyEvent) -> Result<()> {
        match &app.state().cursor_mode() {
            CursorMode::View(_) => match key.code {
                KeyCode::Char('1') => {
                    app.state_mut().goto_login();
                    app.state_mut().set_prompt_message(None);
                }
                KeyCode::Char('q') => app.state_mut().goto_exit(),
                _ => {}
            },
            CursorMode::Edit(_) => match key.code {
                KeyCode::Enter => {
                    if self.login.is_empty() || self.password.is_empty() {
                        app.state_mut()
                            .set_prompt_message(Some(Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "Username or password cannot be empty".to_string(),
                            ))));
                    } else {
                        let response = app.database().create_user(&self.login, &self.password);

                        match response {
                            Ok(_) => {
                                app.state_mut()
                                    .set_prompt_message(Some(Ok("User created".to_string())));
                                app.state_mut().goto_login();
                            }
                            Err(err) => {
                                app.state_mut()
                                    .set_prompt_message(Some(Err(std::io::Error::new(
                                        std::io::ErrorKind::Other,
                                        format!("Failed to fetch chats. {:?}", err.to_string()),
                                    ))));
                            }
                        }
                    }
                }

                KeyCode::Char(c) => match app.state().focus_on() {
                    Some(FocusOn::Line(0, _)) => self.login.push(c),
                    Some(FocusOn::Line(1, _)) => self.password.push(c),
                    _ => app.state_mut().set_focus_on(Some(FocusOn::Line(0, 0))),
                },

                KeyCode::Backspace => match app.state().focus_on() {
                    Some(FocusOn::Line(0, _)) => {
                        self.login.pop();
                    }
                    Some(FocusOn::Line(1, _)) => {
                        self.password.pop();
                    }
                    _ => app.state_mut().set_focus_on(Some(FocusOn::Line(0, 0))),
                },

                KeyCode::Tab => match app.state().focus_on() {
                    Some(FocusOn::Line(0, _)) => {
                        app.state_mut().set_focus_on(Some(FocusOn::Line(1, 0)))
                    }
                    Some(FocusOn::Line(2, _)) => {
                        app.state_mut().set_focus_on(Some(FocusOn::Line(0, 0)))
                    }
                    _ => app.state_mut().set_focus_on(Some(FocusOn::Line(0, 0))),
                },

                _ => {}
            },
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
                    Constraint::Max(3),
                    Constraint::Length(1),
                    Constraint::Max(3),
                    Constraint::Length(1),
                    Constraint::Length(2),
                ]
                .as_ref(),
            )
            .split(frame.area());

        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn handle_resize(&mut self, _: &mut App, _: (u16, u16)) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn setup(&mut self, app: &mut App) -> Result<()> {
        Ok(())
    }
}
