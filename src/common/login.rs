use std::{
    io::{Result, Stdout},
    rc::Rc,
};

use crud_bd::crud::user;
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

pub struct Login {
    pub chunks: Rc<[Rect]>,
    pub login: String,
    pub password: String,
}

impl Login {
    pub fn new() -> Login {
        Login {
            chunks: Rc::new([]),
            login: String::new(),
            password: String::new(),
        }
    }
}

impl Page<CrosstermBackend<Stdout>> for Login {
    fn render(&self, frame: &mut Frame, state: &mut State) -> Result<()> {
        frame.render_widget(
            Paragraph::new(Text::raw("Username:").centered())
                .alignment(Alignment::Center)
                .block(
                    Block::new()
                        .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                        .title("Login"),
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
            Paragraph::new(
                Text::raw(
                    self.password
                        .clone()
                        .chars()
                        .map(|_| '*')
                        .collect::<String>(),
                )
                .centered(),
            )
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

        // render message of error if any on chunk 5 put a dummy message for now

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
                Paragraph::new("Press '2' to switch to Register | Press 'q' to exit")
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
                KeyCode::Char('2') => {
                    app.state_mut().goto_register();
                    app.state_mut().set_prompt_message(None);
                }
                KeyCode::Char('q') => app.state_mut().goto_exit(),
                _ => {}
            },
            CursorMode::Edit(_) => match key.code {
                KeyCode::Enter => {
                    // TODO AUTH USER
                    let response: Result<user::User> = Ok(user::User {
                        id: 0,
                        username: "admin".to_string(),
                        password: "admin".to_string(),
                    });
                    match response {
                        Ok(user) => {
                            if self.login == user.username && self.password == user.password {
                                app.state_mut().goto_client_chats();
                                app.state_mut().set_prompt_message(None);
                            } else {
                                app.state_mut()
                                    .set_prompt_message(Some(Err(std::io::Error::new(
                                        std::io::ErrorKind::Other,
                                        "Invalid password.",
                                    ))));
                            }
                        }
                        Err(_) => {
                            app.state_mut()
                                .set_prompt_message(Some(Err(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    "Invalid username.",
                                ))));
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
                    Some(FocusOn::Line(1, _)) => {
                        app.state_mut().set_focus_on(Some(FocusOn::Line(0, 0)))
                    }
                    _ => app.state_mut().set_focus_on(Some(FocusOn::Line(1, 0))),
                },
                _ => {}
            },
        };
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
