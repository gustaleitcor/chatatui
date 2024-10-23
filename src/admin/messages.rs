use std::{
    collections::HashMap,
    io::{Result, Stdout},
    rc::Rc,
};

use chrono::{NaiveDateTime, Utc};
use crud_bd::crud::message::Message;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::{
    app::{App, CursorMode, FocusOn},
    state::State,
};

use crate::page::Page;

struct Filter {
    id: String,
    content: String,
    chat_id: String,
    user_id: String,
    message_date: String,
}

struct MessageStr {
    id: i32,
    content: String,
    #[allow(dead_code)]
    participant_id: i32,
    chat_id: String,
    user_id: String,
    date: NaiveDateTime,
}

pub struct Messages {
    chunks: Rc<[Rect]>,
    db_cursor: i64,
    messages: Vec<MessageStr>,
    new_message: MessageStr,
    participant_id_to_pair_chat_user_id: HashMap<i32, (i32, i32)>,
    filter: Filter,
    available_rows: i64,
}

impl Messages {
    pub fn new() -> Self {
        Self {
            chunks: Rc::new([Rect::default()]),
            db_cursor: 0,
            messages: Vec::new(),
            new_message: MessageStr {
                id: 0,
                content: String::new(),
                participant_id: 0,
                chat_id: String::new(),
                user_id: String::new(),
                date: Utc::now().naive_utc(),
            },
            participant_id_to_pair_chat_user_id: HashMap::new(),

            filter: Filter {
                id: String::new(),
                content: String::new(),
                chat_id: String::new(),
                user_id: String::new(),
                message_date: String::new(),
            },
            available_rows: 0,
        }
    }
}

impl From<Message> for MessageStr {
    fn from(s: Message) -> MessageStr {
        // if user_id is none set to "NULL" else set to user_id in string format.
        MessageStr {
            id: s.id,
            content: s.content,
            participant_id: s.participant_id,
            user_id: String::new(),
            chat_id: String::new(),
            date: s.date,
        }
    }
}

impl Page<CrosstermBackend<Stdout>> for Messages {
    fn render(&self, frame: &mut Frame, state: &mut State) -> Result<()> {
        // Renders header
        frame.render_widget(
            Paragraph::new("Messages")
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                        .title(format!(
                            " Page: {} ",
                            self.db_cursor.checked_div(self.available_rows).unwrap_or(0),
                        )),
                ),
            self.chunks[0],
        );

        // Renders the filter input.
        let filter_clone = [Filter {
            id: self.filter.id.clone(),
            content: self.filter.content.clone(),
            chat_id: self.filter.chat_id.clone(),
            user_id: self.filter.user_id.clone(),
            message_date: self.filter.message_date.clone(),
        }];

        let row = filter_clone.iter().map(|data| {
            if let CursorMode::View('f') = state.cursor_mode() {
                if let Some(FocusOn::Filter(col)) = state.focus_on() {
                    let mut cells = vec![];
                    for (j, cell) in [
                        data.id.to_owned(),
                        data.content.to_owned(),
                        data.chat_id.to_owned(),
                        data.user_id.to_owned(),
                        data.message_date.to_owned(),
                    ]
                    .iter()
                    .enumerate()
                    {
                        let cell = Cell::from(cell.to_string()).style(if j == *col {
                            Style::default().fg(Color::Black).bg(Color::White)
                        } else {
                            Style::default()
                        });
                        cells.push(cell);
                    }
                    return Row::new(cells).height(1);
                }
            }
            Row::new(vec![
                Cell::from(data.id.to_owned()),
                Cell::from(data.content.to_owned()),
                Cell::from(data.chat_id.to_owned()),
                Cell::from(data.user_id.to_owned()),
                Cell::from(data.message_date.to_owned()),
            ])
            .height(1)
        });

        frame.render_widget(
            Table::new(
                row,
                [
                    // + 1 is for padding.
                    Constraint::Length(9),
                    Constraint::Fill(1),
                    Constraint::Max(8),
                    Constraint::Max(8),
                    Constraint::Max(27),
                ],
            )
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
            self.chunks[1],
        );

        // Renders the table.

        let header = ["Id", "Content", "Chat Id", "User Id", "Date"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(Style::default().fg(Color::Yellow))
            .height(1);

        let rows = self.messages.iter().enumerate().map(|(i, data)| {
            // if let CursorMode::Edit('u') = state.cursor_mode() {
            //     if let Some(FocusOn::Line(row, col)) = state.focus_on() {
            //         if i == *row {
            //             let mut cells = vec![];
            //             for (j, cell) in [
            //                 data.id.to_string(),
            //                 data.content.to_owned(),
            //                 data.chat_id.to_owned(),
            //                 data.user_id.to_owned(),
            //                 data.date.and_utc().to_rfc3339(),
            //             ]
            //             .iter()
            //             .enumerate()
            //             {
            //                 if j == *col {
            //                     cells.push(
            //                         Cell::from(Text::from(cell.to_owned()))
            //                             .style(Style::default().bg(Color::LightBlue)),
            //                     );
            //                 } else {
            //                     cells.push(Cell::from(Text::from(cell.to_owned())));
            //                 }
            //             }
            //             return Row::new(cells);
            //         }
            //     }
            // }

            if let CursorMode::Edit('c') = state.cursor_mode() {
                if let Some(FocusOn::Line(row, col)) = state.focus_on() {
                    if i == *row {
                        let mut cells = vec![];
                        for (j, cell) in [
                            "New Msg:".to_string(),
                            self.new_message.content.to_owned(),
                            self.new_message.chat_id.to_owned(),
                            self.new_message.user_id.to_owned(),
                            Utc::now().to_rfc3339(),
                        ]
                        .iter()
                        .enumerate()
                        {
                            if j == *col {
                                cells.push(
                                    Cell::from(Text::from(cell.to_owned()))
                                        .style(Style::default().bg(Color::LightBlue)),
                                );
                            } else {
                                cells.push(Cell::from(Text::from(cell.to_owned())));
                            }
                        }
                        return Row::new(cells);
                    }
                }
            }

            let id = Cell::from(Text::from(format!("{}", data.id)));
            let content = Cell::from(Text::from(data.content.to_owned()));
            let chat_id = Cell::from(Text::from(
                self.participant_id_to_pair_chat_user_id
                    .get(&data.participant_id)
                    .unwrap_or(&(0, 0))
                    .0
                    .to_string(),
            ));
            let user_id = Cell::from(Text::from(
                self.participant_id_to_pair_chat_user_id
                    .get(&data.participant_id)
                    .unwrap_or(&(0, 0))
                    .1
                    .to_string(),
            ));
            let date = Cell::from(Text::from(data.date.to_string()));

            [id, content, chat_id, user_id, date]
                .into_iter()
                .collect::<Row>()
        });

        frame.render_stateful_widget(
            Table::new(
                rows,
                [
                    // + 1 is for padding.
                    Constraint::Length(9),
                    Constraint::Fill(1),
                    Constraint::Max(8),
                    Constraint::Max(8),
                    Constraint::Length(27),
                ],
            )
            .header(header)
            .highlight_symbol(" >> ")
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
            self.chunks[2],
            &mut TableState::new().with_selected(
                if let Some(FocusOn::Line(n, _)) = state.focus_on() {
                    Some(*n)
                } else {
                    Some(0)
                },
            ),
        );

        if let Some(Ok(msg)) = state.prompt_message() {
            frame.render_widget(
                Paragraph::new(msg.to_owned())
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
                self.chunks[3],
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
                self.chunks[3],
            );
        } else {
            frame.render_widget(
                Block::default().borders(Borders::LEFT | Borders::RIGHT),
                self.chunks[3],
            );
        };

        let guide_block = Block::default()
            .borders(Borders::TOP)
            .title(state.cursor_mode().as_str());

        let guide_align = Alignment::Center;

        let guide_content = match state.cursor_mode() {
            CursorMode::Edit('c') => "Press 'Enter' to confirm | Press 'esc' to cancel",
            // CursorMode::Edit('u') => "Press 'Enter' to confirm | Press 'esc' to cancel",
            CursorMode::Edit('d') => "Press 'y' to confirm | Press 'esc' to cancel",
            CursorMode::View(_) => "Press 'Esc' to toggle to Edit Mode | Press 'f' to filter | Press 'q' to goto menu",
            CursorMode::Edit(_) => "Press 'Esc' to toggle to View Mode | Press 'c' to create | Press 'd' to delete | Press 'u' to update | Press 'q' to goto menu",
        };

        frame.render_widget(
            Paragraph::new(guide_content)
                .alignment(guide_align)
                .block(guide_block),
            self.chunks[4],
        );

        Ok(())
    }
    fn layout(&mut self, frame: &mut Frame) -> Result<()> {
        self.chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Fill(1),
                    Constraint::Length(2),
                    Constraint::Length(2),
                ]
                .as_ref(),
            )
            .split(frame.area());

        self.available_rows = self.chunks[2].height.saturating_sub(1) as i64;

        Ok(())
    }
    fn handle_input(&mut self, app: &mut App, key: &KeyEvent) -> Result<()> {
        match app.state().cursor_mode() {
            CursorMode::View('x') => match key.code {
                KeyCode::Char('q') => {
                    app.state_mut().goto_menu();
                }
                KeyCode::Down => {
                    if let Some(FocusOn::Line(n, _)) = app.state().focus_on().clone() {
                        if !self.messages.is_empty() {
                            if n < self.messages.len() - 1 {
                                app.state_mut().set_focus_on(Some(FocusOn::Line(n + 1, 1)));
                            } else {
                                // TODO: This is disgusting

                                let messages = app
                                    .database()
                                    .next_messages_page(
                                        self.available_rows,
                                        &mut self.db_cursor,
                                        Some(self.filter.id.clone()),
                                        Some(self.filter.user_id.clone()),
                                        Some(self.filter.chat_id.clone()),
                                        Some(self.filter.message_date.clone()),
                                    )
                                    .unwrap();

                                self.participant_id_to_pair_chat_user_id = messages.1;

                                if !messages.0.is_empty() {
                                    self.messages =
                                        messages.0.into_iter().map(|m| m.into()).collect();
                                    app.state_mut().set_focus_on(Some(FocusOn::Line(0, 1)));
                                }
                            }
                        } else {
                            app.state_mut().set_focus_on(None);
                        }
                    }
                }

                KeyCode::Up => {
                    if let Some(FocusOn::Line(n, _)) = app.state().focus_on().clone() {
                        if !self.messages.is_empty() {
                            if n > 0 {
                                app.state_mut().set_focus_on(Some(FocusOn::Line(n - 1, 1)));
                            } else if !(self.db_cursor == 0 && n == 0) {
                                let chats = app
                                    .database()
                                    .prev_messages_page(
                                        self.available_rows,
                                        &mut self.db_cursor,
                                        Some(self.filter.id.clone()),
                                        Some(self.filter.user_id.clone()),
                                        Some(self.filter.chat_id.clone()),
                                        Some(self.filter.message_date.clone()),
                                    )
                                    .unwrap();

                                self.participant_id_to_pair_chat_user_id = chats.1;

                                self.messages = chats.0.into_iter().map(|m| m.into()).collect();
                                app.state_mut()
                                    .set_focus_on(Some(FocusOn::Line(self.messages.len() - 1, 1)));
                            }
                        } else {
                            app.state_mut().set_focus_on(None);
                        }
                    }
                }

                KeyCode::Char('f') => {
                    app.state_mut().set_focus_on(Some(FocusOn::Filter(0)));
                    app.state_mut().set_cursor_mode(CursorMode::View('f'));
                }

                _ => {}
            },

            CursorMode::View('f') => {
                if let Some(FocusOn::Filter(n)) = app.state().focus_on().clone() {
                    match key.code {
                        KeyCode::Char(c) => {
                            match n {
                                0 => {
                                    self.filter.id.push(c);
                                }

                                1 => {
                                    self.filter.content.push(c);
                                }

                                2 => {
                                    self.filter.chat_id.push(c);
                                }

                                3 => {
                                    self.filter.user_id.push(c);
                                }

                                4 => {
                                    self.filter.message_date.push(c);
                                }

                                _ => {}
                            }

                            self.db_cursor = 0;

                            let a;
                            (a, self.participant_id_to_pair_chat_user_id) = app
                                .database()
                                .fetch_messages(
                                    self.available_rows,
                                    self.db_cursor,
                                    Some(self.filter.id.clone()),
                                    Some(self.filter.user_id.clone()),
                                    Some(self.filter.chat_id.clone()),
                                    Some(self.filter.message_date.clone()),
                                )
                                .unwrap();

                            self.messages = a.into_iter().map(|m| m.into()).collect();
                        }

                        KeyCode::Backspace => {
                            match n {
                                0 => {
                                    self.filter.id.pop();
                                }

                                1 => {
                                    self.filter.content.pop();
                                }

                                2 => {
                                    self.filter.chat_id.pop();
                                }

                                3 => {
                                    self.filter.user_id.pop();
                                }

                                4 => {
                                    self.filter.message_date.pop();
                                }

                                _ => {}
                            }

                            self.db_cursor = 0;

                            let mut a;
                            (a, self.participant_id_to_pair_chat_user_id) = app
                                .database()
                                .fetch_messages(
                                    self.available_rows,
                                    self.db_cursor,
                                    Some(self.filter.id.clone()),
                                    Some(self.filter.user_id.clone()),
                                    Some(self.filter.chat_id.clone()),
                                    Some(self.filter.message_date.clone()),
                                )
                                .unwrap();

                            self.messages = a.into_iter().map(|m| m.into()).collect();
                        }

                        KeyCode::Right => {
                            if n < 4 {
                                app.state_mut().set_focus_on(Some(FocusOn::Filter(n + 1)));
                            }
                        }

                        KeyCode::Left => {
                            if n > 0 {
                                app.state_mut().set_focus_on(Some(FocusOn::Filter(n - 1)));
                            }
                        }

                        KeyCode::Down => {
                            app.state_mut().set_cursor_mode(CursorMode::View('x'));
                            app.state_mut().set_focus_on(Some(FocusOn::Line(0, 1)));
                        }

                        _ => {}
                    }
                }
            }

            CursorMode::Edit('x') => match key.code {
                KeyCode::Char('q') => {
                    app.state_mut().goto_menu();
                }

                KeyCode::Down => {
                    if let Some(FocusOn::Line(n, _)) = app.state().focus_on().clone() {
                        if !self.messages.is_empty() {
                            if n < self.messages.len() - 1 {
                                app.state_mut().set_focus_on(Some(FocusOn::Line(n + 1, 1)));
                            } else {
                                // TODO: This is disgusting

                                let chats = app
                                    .database()
                                    .next_messages_page(
                                        self.available_rows,
                                        &mut self.db_cursor,
                                        Some(self.filter.id.clone()),
                                        Some(self.filter.user_id.clone()),
                                        Some(self.filter.chat_id.clone()),
                                        Some(self.filter.message_date.clone()),
                                    )
                                    .unwrap();

                                self.participant_id_to_pair_chat_user_id = chats.1;

                                if !chats.0.is_empty() {
                                    self.messages = chats.0.into_iter().map(|m| m.into()).collect();
                                    app.state_mut().set_focus_on(Some(FocusOn::Line(0, 1)));
                                }
                            }
                        } else {
                            app.state_mut().set_focus_on(None);
                        }
                    }
                }

                KeyCode::Up => {
                    if let Some(FocusOn::Line(n, _)) = app.state().focus_on().clone() {
                        if !self.messages.is_empty() {
                            if n > 0 {
                                app.state_mut().set_focus_on(Some(FocusOn::Line(n - 1, 1)));
                            } else if !(self.db_cursor == 0 && n == 0) {
                                let chats = app
                                    .database()
                                    .prev_messages_page(
                                        self.available_rows,
                                        &mut self.db_cursor,
                                        Some(self.filter.id.clone()),
                                        Some(self.filter.user_id.clone()),
                                        Some(self.filter.chat_id.clone()),
                                        Some(self.filter.message_date.clone()),
                                    )
                                    .unwrap();

                                self.participant_id_to_pair_chat_user_id = chats.1;

                                let chats = chats.0.into_iter().map(|m| m.into()).collect();

                                self.messages = chats;
                                app.state_mut()
                                    .set_focus_on(Some(FocusOn::Line(self.messages.len() - 1, 1)));
                            }
                        } else {
                            app.state_mut().set_focus_on(None);
                        }
                    }
                }

                KeyCode::Char('d') => {
                    if !self.messages.is_empty() {
                        if let Some(FocusOn::Line(row, _)) = app.state().focus_on() {
                            if self.messages.get(*row).is_some() {
                                app.state_mut().set_cursor_mode(CursorMode::Edit('d'));
                            }
                        }
                    }
                }

                // KeyCode::Char('u') => {
                //     if !self.messages.is_empty() {
                //         if let Some(FocusOn::Line(row, _)) = app.state().focus_on() {
                //             if self.messages.get(*row).is_some() {
                //                 app.state_mut().set_cursor_mode(CursorMode::Edit('u'));
                //             }
                //         }
                //     }
                // }
                KeyCode::Char('c') => {
                    app.state_mut().set_cursor_mode(CursorMode::Edit('c'));
                    self.messages.push(MessageStr {
                        id: -1,
                        user_id: "".to_string(),
                        chat_id: "".to_string(),
                        participant_id: 0,
                        content: "".to_string(),
                        date: Utc::now().naive_utc(),
                    });
                    app.state_mut()
                        .set_focus_on(Some(FocusOn::Line(self.messages.len() - 1, 1)));
                }

                _ => {}
            },

            CursorMode::Edit('d') => {
                if key.code == KeyCode::Char('y') {
                    if let Some(FocusOn::Line(n, _)) = app.state().focus_on().clone() {
                        if !self.messages.is_empty() {
                            let message = self.messages.get(n).unwrap();
                            let message_id = message.id.to_owned();
                            match app.database().delete_message(message_id) {
                                Ok(_) => {
                                    self.messages.remove(n);
                                    let mut a;
                                    (a, self.participant_id_to_pair_chat_user_id) = app
                                        .database()
                                        .fetch_messages(
                                            self.available_rows,
                                            self.db_cursor,
                                            Some(self.filter.id.clone()),
                                            Some(self.filter.user_id.clone()),
                                            Some(self.filter.chat_id.clone()),
                                            Some(self.filter.message_date.clone()),
                                        )
                                        .unwrap();

                                    self.messages = a.into_iter().map(|m| m.into()).collect();
                                    app.state_mut()
                                        .set_focus_on(Some(FocusOn::Line(n.saturating_sub(1), 1)));
                                    app.state_mut()
                                        .set_prompt_message(Some(Ok("Message deleted".to_string())))
                                }

                                Err(err) => app.state_mut().set_prompt_message(Some(Err(
                                    std::io::Error::new(
                                        std::io::ErrorKind::Other,
                                        format!("Failed to delete message. {:?}", err),
                                    ),
                                ))),
                            }
                        }
                    }

                    app.state_mut().set_cursor_mode(CursorMode::Edit('x'));
                }
            }

            // SAY NO TO CENSORSHIP

            // CursorMode::Edit('u') => {
            //     if let Some(FocusOn::Line(row, col)) = app.state().focus_on().clone() {
            //         match key.code {
            //             KeyCode::Enter => {
            //                 if let Some(chat) = self.messages.get_mut(row) {
            //                     let chat_id = &chat.id.to_owned();
            //                     let chat_title = &chat.title.to_owned();
            //                     let chat_is_public = chat.is_public;

            //                     // TODO: handle error
            //                     match app.database().update_chat_title(*chat_id, chat_title) {
            //                         Ok(_) => app
            //                             .state_mut()
            //                             .set_prompt_message(Some(Ok("Chat created".to_string()))),
            //                         Err(err) => {
            //                             self.messages = app
            //                                 .database()
            //                                 .fetch_messages(
            //                                     self.available_rows,
            //                                     self.db_cursor,
            //                                     Some(self.filter.id.clone()),
            //                                     Some(self.filter.user_id.clone()),
            //                                     Some(self.filter.chat_id.clone()),
            //                                     Some(self.filter.message_date.clone()),
            //                                 )
            //                                 .unwrap()
            //                                 .into_iter()
            //                                 .map(|m| m.into())
            //                                 .collect();
            //                             app.state_mut().set_prompt_message(Some(Err(
            //                                 std::io::Error::new(
            //                                     std::io::ErrorKind::Other,
            //                                     format!(
            //                                         "Failed to create chat title. {:?}",
            //                                         err.to_string()
            //                                     ),
            //                                 ),
            //                             )));
            //                         }
            //                     }

            //                     // TODO: handle errors:
            //                     match app.database().update_chat_privacy(*chat_id, chat_is_public) {
            //                         Ok(_) => app
            //                             .state_mut()
            //                             .set_prompt_message(Some(Ok("Chat updated".to_string()))),
            //                         Err(err) => {
            //                             self.messages = app
            //                                 .database()
            //                                 .fetch_chats(
            //                                     self.available_rows,
            //                                     self.db_cursor,
            //                                     Some(self.filter.title.clone()),
            //                                     Some(self.filter.id.clone()),
            //                                 )
            //                                 .unwrap();
            //                             app.state_mut().set_prompt_message(Some(Err(
            //                                 std::io::Error::new(
            //                                     std::io::ErrorKind::Other,
            //                                     format!(
            //                                         "Failed to update user password. {:?}",
            //                                         err,
            //                                     ),
            //                                 ),
            //                             )));
            //                             // return;
            //                         }
            //                     }
            //                 }

            //                 app.state_mut().toggle_cursor_mode();
            //                 app.state_mut().set_focus_on(Some(FocusOn::Line(row, 1)));
            //             }

            //             KeyCode::Char(c) => {
            //                 if let Some(chat) = self.messages.get_mut(row) {
            //                     if col == 1 {
            //                         chat.title.push(c);
            //                     }
            //                 }
            //             }

            //             KeyCode::Backspace => {
            //                 if let Some(user) = self.messages.get_mut(row) {
            //                     if col == 1 {
            //                         user.title.pop();
            //                     }
            //                 }
            //             }

            //             KeyCode::Left => {
            //                 if col > 1 {
            //                     app.state_mut()
            //                         .set_focus_on(Some(FocusOn::Line(row, col - 1)));
            //                 }
            //             }

            //             KeyCode::Right => {
            //                 if col < 2 {
            //                     app.state_mut()
            //                         .set_focus_on(Some(FocusOn::Line(row, col + 1)));
            //                 }
            //             }

            //             KeyCode::Up => {
            //                 if let Some(user) = self.messages.get_mut(row) {
            //                     if col == 2 {
            //                         user.is_public = !user.is_public;
            //                     }
            //                 }
            //             }

            //             KeyCode::Down => {
            //                 if let Some(user) = self.messages.get_mut(row) {
            //                     if col == 2 {
            //                         user.is_public = !user.is_public;
            //                     }
            //                 }
            //             }

            //             _ => (),
            //         }
            //     }
            // }
            //
            CursorMode::Edit('c') => {
                if let Some(FocusOn::Line(row, col)) = app.state().focus_on().clone() {
                    match key.code {
                        KeyCode::Enter => {
                            // TODO: handle parse error
                            match app.database().create_message(
                                self.new_message.user_id.parse::<i32>().unwrap(),
                                self.new_message.chat_id.parse::<i32>().unwrap(),
                                self.new_message.content.as_str(),
                            ) {
                                Ok(_) => {
                                    app.state_mut().set_prompt_message(Some(Ok(
                                        "Message created".to_string()
                                    )));
                                }
                                Err(err) => app.state_mut().set_prompt_message(Some(Err(
                                    std::io::Error::new(
                                        std::io::ErrorKind::Other,
                                        format!("Failed to create message. {:?}", err),
                                    ),
                                ))),
                            }

                            app.state_mut().set_cursor_mode(CursorMode::Edit('x'));
                            app.state_mut().set_focus_on(Some(FocusOn::Line(0, 1)));
                            // TODO: This can lead to errors
                            let mut a;
                            (a, self.participant_id_to_pair_chat_user_id) = app
                                .database()
                                .fetch_messages(
                                    self.available_rows,
                                    self.db_cursor,
                                    Some(self.filter.id.clone()),
                                    Some(self.filter.user_id.clone()),
                                    Some(self.filter.chat_id.clone()),
                                    Some(self.filter.message_date.clone()),
                                )
                                .unwrap();

                            self.messages = a.into_iter().map(|m| m.into()).collect();
                            self.new_message.content.clear();
                            self.new_message.chat_id.clear();
                            self.new_message.user_id.clear();
                        }

                        KeyCode::Char(c) => match col {
                            1 => {
                                self.new_message.content.push(c);
                            }

                            2 => {
                                self.new_message.chat_id.push(c);
                            }

                            3 => {
                                self.new_message.user_id.push(c);
                            }

                            _ => {}
                        },

                        KeyCode::Backspace => match col {
                            1 => {
                                self.new_message.content.pop();
                            }

                            2 => {
                                self.new_message.chat_id.pop();
                            }

                            3 => {
                                self.new_message.user_id.pop();
                            }

                            _ => (),
                        },

                        KeyCode::Left => {
                            if col > 1 {
                                app.state_mut()
                                    .set_focus_on(Some(FocusOn::Line(row, col - 1)));
                            }
                        }

                        KeyCode::Right => {
                            if col < 3 {
                                app.state_mut()
                                    .set_focus_on(Some(FocusOn::Line(row, col + 1)));
                            }
                        }

                        _ => (),
                    }
                }
            }

            _ => {}
        }
        Ok(())
    }
    fn handle_resize(&mut self, app: &mut App, _: (u16, u16)) -> Result<()> {
        if let CursorMode::Edit('c') = app.state().cursor_mode() {
            return Ok(());
        }

        self.messages = match app.database().fetch_messages(
            self.available_rows,
            self.db_cursor,
            Some(self.filter.id.clone()),
            Some(self.filter.user_id.clone()),
            Some(self.filter.chat_id.clone()),
            Some(self.filter.message_date.clone()),
        ) {
            Ok(chats) => {
                self.participant_id_to_pair_chat_user_id = chats.1;
                chats.0.into_iter().map(|c| c.into()).collect()
            }
            Err(err) => {
                app.state_mut()
                    .set_prompt_message(Some(Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to fetch messages. {:?}", err.to_string()),
                    ))));

                self.db_cursor = 0;
                Vec::new()
            }
        };

        if let Some(FocusOn::Line(n, _)) = app.state().focus_on() {
            if !self.messages.is_empty() {
                if *n >= self.messages.len() {
                    app.state_mut()
                        .set_focus_on(Some(FocusOn::Line(self.messages.len() - 1, 1)));
                } else if self.messages.len() <= self.chunks[1].height.saturating_sub(1) as usize {
                    self.db_cursor = (self.db_cursor as u64).saturating_sub(
                        1 + (self.chunks[1].height.saturating_sub(1) as usize - self.messages.len())
                            as u64,
                    ) as i64;
                }
            }
        }
        Ok(())
    }
    fn cleanup(&mut self) -> Result<()> {
        // cleanup users
        // cleanup new user form
        // cleanup pagination
        Ok(())
    }

    fn setup(&mut self, app: &mut App) -> Result<()> {
        self.messages = match app.database().fetch_messages(
            self.available_rows,
            self.db_cursor,
            Some(self.filter.id.clone()),
            Some(self.filter.user_id.clone()),
            Some(self.filter.chat_id.clone()),
            Some(self.filter.message_date.clone()),
        ) {
            Ok(chats) => {
                self.participant_id_to_pair_chat_user_id = chats.1;
                chats.0.into_iter().map(|c| c.into()).collect()
            }
            Err(err) => {
                app.state_mut()
                    .set_prompt_message(Some(Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to fetch messages. {:?}", err.to_string()),
                    ))));

                self.db_cursor = 0;
                Vec::new()
            }
        };

        if self.messages.is_empty() {
            app.state_mut().set_focus_on(None);
        } else {
            match app.state().focus_on() {
                Some(FocusOn::Line(n, _)) => {
                    if *n >= self.messages.len() {
                        app.state_mut()
                            .set_focus_on(Some(FocusOn::Line(self.messages.len() - 1, 1)));
                    } else if self.messages.len()
                        <= self.chunks[1].height.saturating_sub(1) as usize
                    {
                        self.db_cursor = (self.db_cursor as u64).saturating_sub(
                            1 + (self.chunks[1].height.saturating_sub(1) as usize
                                - self.messages.len()) as u64,
                        ) as i64;
                    }
                }
                _ => {
                    app.state_mut().set_focus_on(Some(FocusOn::Line(0, 1)));
                }
            }
        }

        Ok(())
    }
}
