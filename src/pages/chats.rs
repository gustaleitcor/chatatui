use std::{
    io::{Result, Stdout},
    rc::Rc,
};

use crud_bd::crud::chat::Chat;
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

use super::page::Page;

struct Filter {
    id: String,
    title: String,
}

pub struct Chats {
    chunks: Rc<[Rect]>,
    db_cursor: i64,
    chats: Vec<Chat>,
    new_chat: Chat,
    filter: Filter,
    available_rows: i64,
}

impl Chats {
    pub fn new() -> Self {
        Self {
            chunks: Rc::new([Rect::default()]),
            db_cursor: 0,
            chats: Vec::new(),
            new_chat: Chat {
                id: -1,
                title: String::new(),
                is_public: false,
            },
            filter: Filter {
                id: String::new(),
                title: String::new(),
            },
            available_rows: 0,
        }
    }
}

impl Page<CrosstermBackend<Stdout>> for Chats {
    fn render(&self, frame: &mut Frame, state: &mut State) -> Result<()> {
        // Renders header
        frame.render_widget(
            Paragraph::new("Chats").alignment(Alignment::Center).block(
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
            id: self.filter.id.to_owned(),
            title: self.filter.title.to_owned(),
        }];

        let row = filter_clone.iter().map(|data| {
            if let CursorMode::View('f') = state.cursor_mode() {
                if let Some(FocusOn::Filter(col)) = state.focus_on() {
                    let mut cells = vec![];
                    for (j, cell) in [data.id.to_owned(), data.title.to_owned()]
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
                Cell::from(data.title.to_owned()),
            ])
            .height(1)
        });

        frame.render_widget(
            Table::new(
                row,
                [
                    // + 1 is for padding.
                    Constraint::Length(9),
                    Constraint::Max(50),
                ],
            )
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
            self.chunks[1],
        );

        // Renders the table.

        let header = ["Id", "Title", "Private"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(Style::default().fg(Color::Yellow))
            .height(1);

        let rows = self.chats.iter().enumerate().map(|(i, data)| {
            if let CursorMode::Edit('u') = state.cursor_mode() {
                if let Some(FocusOn::Line(row, col)) = state.focus_on() {
                    if i == *row {
                        let mut cells = vec![];
                        for (j, cell) in [
                            data.id.to_string(),
                            data.title.to_owned(),
                            data.is_public.to_string(),
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

            if let CursorMode::Edit('c') = state.cursor_mode() {
                if let Some(FocusOn::Line(row, col)) = state.focus_on() {
                    if i == *row {
                        let mut cells = vec![];
                        for (j, cell) in [
                            "New Chat:".to_string(),
                            self.new_chat.title.to_owned(),
                            self.new_chat.is_public.to_string().to_owned(),
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
            let username = Cell::from(Text::from(data.title.to_owned()));
            let password = Cell::from(Text::from(data.is_public.to_string().to_owned()));

            [id, username, password].into_iter().collect::<Row>()
        });

        frame.render_stateful_widget(
            Table::new(
                rows,
                [
                    // + 1 is for padding.
                    Constraint::Length(9),
                    Constraint::Max(50),
                    Constraint::Max(8),
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
            CursorMode::Edit('u') => "Press 'Enter' to confirm | Press 'esc' to cancel",
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
                        if !self.chats.is_empty() {
                            if n < self.chats.len() - 1 {
                                app.state_mut().set_focus_on(Some(FocusOn::Line(n + 1, 1)));
                            } else {
                                // TODO: This is disgusting

                                let chats = app
                                    .database()
                                    .next_chats_page(
                                        self.available_rows,
                                        &mut self.db_cursor,
                                        Some(self.filter.title.clone()),
                                        Some(self.filter.id.clone()),
                                    )
                                    .unwrap();

                                if !chats.is_empty() {
                                    self.chats = chats;
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
                        if !self.chats.is_empty() {
                            if n > 0 {
                                app.state_mut().set_focus_on(Some(FocusOn::Line(n - 1, 1)));
                            } else if !(self.db_cursor == 0 && n == 0) {
                                let chats = app
                                    .database()
                                    .prev_chats_page(
                                        self.available_rows,
                                        &mut self.db_cursor,
                                        Some(self.filter.title.clone()),
                                        Some(self.filter.id.clone()),
                                    )
                                    .unwrap();

                                self.chats = chats;
                                app.state_mut()
                                    .set_focus_on(Some(FocusOn::Line(self.chats.len() - 1, 1)));
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
                                    self.filter.title.push(c);
                                }

                                _ => {}
                            }

                            self.db_cursor = 0;

                            self.chats = app
                                .database()
                                .fetch_chats(
                                    self.available_rows,
                                    self.db_cursor,
                                    Some(self.filter.title.clone()),
                                    Some(self.filter.id.clone()),
                                )
                                .unwrap();
                        }

                        KeyCode::Backspace => {
                            match n {
                                0 => {
                                    self.filter.id.pop();
                                }

                                1 => {
                                    self.filter.title.pop();
                                }

                                _ => {}
                            }

                            self.db_cursor = 0;

                            self.chats = app
                                .database()
                                .fetch_chats(
                                    self.available_rows,
                                    self.db_cursor,
                                    Some(self.filter.title.clone()),
                                    Some(self.filter.id.clone()),
                                )
                                .unwrap();
                        }

                        KeyCode::Right => {
                            if n < 1 {
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
                        if !self.chats.is_empty() {
                            if n < self.chats.len() - 1 {
                                app.state_mut().set_focus_on(Some(FocusOn::Line(n + 1, 1)));
                            } else {
                                // TODO: This is disgusting

                                let chats = app
                                    .database()
                                    .next_chats_page(
                                        self.available_rows,
                                        &mut self.db_cursor,
                                        Some(self.filter.title.clone()),
                                        Some(self.filter.id.clone()),
                                    )
                                    .unwrap();

                                if !chats.is_empty() {
                                    self.chats = chats;
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
                        if !self.chats.is_empty() {
                            if n > 0 {
                                app.state_mut().set_focus_on(Some(FocusOn::Line(n - 1, 1)));
                            } else if !(self.db_cursor == 0 && n == 0) {
                                let chats = app
                                    .database()
                                    .prev_chats_page(
                                        self.available_rows,
                                        &mut self.db_cursor,
                                        Some(self.filter.title.clone()),
                                        Some(self.filter.id.clone()),
                                    )
                                    .unwrap();
                                self.chats = chats;
                                app.state_mut()
                                    .set_focus_on(Some(FocusOn::Line(self.chats.len() - 1, 1)));
                            }
                        } else {
                            app.state_mut().set_focus_on(None);
                        }
                    }
                }

                KeyCode::Char('d') => {
                    if !self.chats.is_empty() {
                        if let Some(FocusOn::Line(row, _)) = app.state().focus_on() {
                            if self.chats.get(*row).is_some() {
                                app.state_mut().set_cursor_mode(CursorMode::Edit('d'));
                            }
                        }
                    }
                }

                KeyCode::Char('u') => {
                    if !self.chats.is_empty() {
                        if let Some(FocusOn::Line(row, _)) = app.state().focus_on() {
                            if self.chats.get(*row).is_some() {
                                app.state_mut().set_cursor_mode(CursorMode::Edit('u'));
                            }
                        }
                    }
                }

                KeyCode::Char('c') => {
                    app.state_mut().set_cursor_mode(CursorMode::Edit('c'));
                    self.chats.push(Chat {
                        id: -1,
                        title: "".to_string(),
                        is_public: false,
                    });
                    app.state_mut()
                        .set_focus_on(Some(FocusOn::Line(self.chats.len() - 1, 1)));
                }

                _ => {}
            },

            CursorMode::Edit('d') => {
                if key.code == KeyCode::Char('y') {
                    if let Some(FocusOn::Line(n, _)) = app.state().focus_on().clone() {
                        if !self.chats.is_empty() {
                            let chat = self.chats.get(n).unwrap();
                            let chat_id = chat.id.to_owned();
                            match app.database().delete_chat(chat_id) {
                                Ok(_) => {
                                    self.chats.remove(n);
                                    self.chats = app
                                        .database()
                                        .fetch_chats(
                                            self.available_rows,
                                            self.db_cursor,
                                            Some(self.filter.title.clone()),
                                            Some(self.filter.id.clone()),
                                        )
                                        .unwrap();
                                    app.state_mut()
                                        .set_focus_on(Some(FocusOn::Line(n.saturating_sub(1), 1)));
                                    app.state_mut()
                                        .set_prompt_message(Some(Ok("Chat deleted".to_string())))
                                }

                                Err(err) => app.state_mut().set_prompt_message(Some(Err(
                                    std::io::Error::new(
                                        std::io::ErrorKind::Other,
                                        format!("Failed to delete chat. {:?}", err),
                                    ),
                                ))),
                            }
                        }
                    }

                    app.state_mut().set_cursor_mode(CursorMode::Edit('x'));
                }
            }

            CursorMode::Edit('u') => {
                if let Some(FocusOn::Line(row, col)) = app.state().focus_on().clone() {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(chat) = self.chats.get_mut(row) {
                                let chat_id = &chat.id.to_owned();
                                let chat_title = &chat.title.to_owned();
                                let chat_is_public = chat.is_public;

                                // TODO: handle error
                                match app.database().update_chat_title(*chat_id, chat_title) {
                                    Ok(_) => app
                                        .state_mut()
                                        .set_prompt_message(Some(Ok("Chat created".to_string()))),
                                    Err(err) => {
                                        self.chats = app
                                            .database()
                                            .fetch_chats(
                                                self.available_rows,
                                                self.db_cursor,
                                                Some(self.filter.title.clone()),
                                                Some(self.filter.id.clone()),
                                            )
                                            .unwrap();
                                        app.state_mut().set_prompt_message(Some(Err(
                                            std::io::Error::new(
                                                std::io::ErrorKind::Other,
                                                format!(
                                                    "Failed to set chat title. {:?}",
                                                    err.to_string()
                                                ),
                                            ),
                                        )));
                                    }
                                }

                                // TODO: handle errors:
                                match app.database().update_chat_privacy(*chat_id, chat_is_public) {
                                    Ok(_) => app
                                        .state_mut()
                                        .set_prompt_message(Some(Ok("Chat updated".to_string()))),
                                    Err(err) => {
                                        self.chats = app
                                            .database()
                                            .fetch_chats(
                                                self.available_rows,
                                                self.db_cursor,
                                                Some(self.filter.title.clone()),
                                                Some(self.filter.id.clone()),
                                            )
                                            .unwrap();
                                        app.state_mut().set_prompt_message(Some(Err(
                                            std::io::Error::new(
                                                std::io::ErrorKind::Other,
                                                format!(
                                                    "Failed to update chat privacy. {:?}",
                                                    err,
                                                ),
                                            ),
                                        )));
                                        // return;
                                    }
                                }
                            }

                            app.state_mut().toggle_cursor_mode();
                            app.state_mut().set_focus_on(Some(FocusOn::Line(row, 1)));
                        }

                        KeyCode::Char(c) => {
                            if let Some(chat) = self.chats.get_mut(row) {
                                if col == 1 {
                                    chat.title.push(c);
                                }
                            }
                        }

                        KeyCode::Backspace => {
                            if let Some(user) = self.chats.get_mut(row) {
                                if col == 1 {
                                    user.title.pop();
                                }
                            }
                        }

                        KeyCode::Left => {
                            if col > 1 {
                                app.state_mut()
                                    .set_focus_on(Some(FocusOn::Line(row, col - 1)));
                            }
                        }

                        KeyCode::Right => {
                            if col < 2 {
                                app.state_mut()
                                    .set_focus_on(Some(FocusOn::Line(row, col + 1)));
                            }
                        }

                        KeyCode::Up => {
                            if let Some(user) = self.chats.get_mut(row) {
                                if col == 2 {
                                    user.is_public = !user.is_public;
                                }
                            }
                        }

                        KeyCode::Down => {
                            if let Some(user) = self.chats.get_mut(row) {
                                if col == 2 {
                                    user.is_public = !user.is_public;
                                }
                            }
                        }

                        _ => (),
                    }
                }
            }

            CursorMode::Edit('c') => {
                if let Some(FocusOn::Line(row, col)) = app.state().focus_on().clone() {
                    match key.code {
                        KeyCode::Enter => {
                            // TODO: handle errors
                            match app
                                .database()
                                .create_chat(&self.new_chat.title, self.new_chat.is_public)
                            {
                                Ok(_) => {
                                    app.state_mut()
                                        .set_prompt_message(Some(Ok("Chat created".to_string())));
                                }

                                Err(err) => app.state_mut().set_prompt_message(Some(Err(
                                    std::io::Error::new(
                                        std::io::ErrorKind::Other,
                                        format!("Failed to create chat. {:?}", err),
                                    ),
                                ))),
                            }

                            app.state_mut().set_cursor_mode(CursorMode::Edit('x'));
                            app.state_mut().set_focus_on(Some(FocusOn::Line(0, 1)));
                            // TODO: This can lead to errors
                            self.chats = app
                                .database()
                                .fetch_chats(
                                    self.available_rows,
                                    self.db_cursor,
                                    Some(self.filter.title.clone()),
                                    Some(self.filter.id.clone()),
                                )
                                .unwrap();
                            self.new_chat.title.clear();
                            self.new_chat.is_public = false;
                        }

                        KeyCode::Char(c) => match col {
                            1 => {
                                self.new_chat.title.push(c);
                            }

                            2 => {
                                // TODO: see how to handle boolean input later
                                // self.new_chat.is_public;
                            }

                            _ => {}
                        },

                        KeyCode::Backspace => match col {
                            1 => {
                                self.new_chat.title.pop();
                            }

                            2 => {
                                // TODO: see how to handle boolean input later
                                // self.new_chat.is_public.pop();
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
                            if col < 2 {
                                app.state_mut()
                                    .set_focus_on(Some(FocusOn::Line(row, col + 1)));
                            }
                        }

                        KeyCode::Up => {
                            if let Some(user) = self.chats.get_mut(row) {
                                if col == 2 {
                                    user.is_public = !user.is_public;
                                }
                            }
                        }

                        KeyCode::Down => {
                            if let Some(user) = self.chats.get_mut(row) {
                                if col == 2 {
                                    user.is_public = !user.is_public;
                                }
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

        self.chats = match app.database().fetch_chats(
            self.available_rows,
            self.db_cursor,
            Some(self.filter.title.to_owned()),
            Some(self.filter.id.to_owned()),
        ) {
            Ok(chats) => chats,
            Err(err) => {
                app.state_mut()
                    .set_prompt_message(Some(Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to fetch chats. {:?}", err.to_string()),
                    ))));

                self.db_cursor = 0;
                Vec::new()
            }
        };

        if let Some(FocusOn::Line(n, _)) = app.state().focus_on() {
            if !self.chats.is_empty() {
                if *n >= self.chats.len() {
                    app.state_mut()
                        .set_focus_on(Some(FocusOn::Line(self.chats.len() - 1, 1)));
                } else if self.chats.len() <= self.chunks[1].height.saturating_sub(1) as usize {
                    self.db_cursor = (self.db_cursor as u64).saturating_sub(
                        1 + (self.chunks[1].height.saturating_sub(1) as usize - self.chats.len())
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
        self.chats = match app.database().fetch_chats(
            self.available_rows,
            self.db_cursor,
            Some(self.filter.title.clone()),
            Some(self.filter.id.clone()),
        ) {
            Ok(chats) => chats,
            Err(err) => {
                app.state_mut()
                    .set_prompt_message(Some(Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to fetch chats. {:?}", err.to_string()),
                    ))));

                self.db_cursor = 0;
                Vec::new()
            }
        };

        if self.chats.is_empty() {
            app.state_mut().set_focus_on(None);
        } else {
            match app.state().focus_on() {
                Some(FocusOn::Line(n, _)) => {
                    if *n >= self.chats.len() {
                        app.state_mut()
                            .set_focus_on(Some(FocusOn::Line(self.chats.len() - 1, 1)));
                    } else if self.chats.len() <= self.chunks[1].height.saturating_sub(1) as usize {
                        self.db_cursor = (self.db_cursor as u64).saturating_sub(
                            1 + (self.chunks[1].height.saturating_sub(1) as usize
                                - self.chats.len()) as u64,
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
