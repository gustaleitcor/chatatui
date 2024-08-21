use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::{Alignment, Constraint, Direction, Layout, Rows},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Cell, List, ListState, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::admin::{Admin, AdminCurrentScreen, AdminFocusOn};
use crate::app::CursorMode;

pub fn ui_admin(f: &mut Frame, app: &mut Admin) {
    let mut current_event = app.take_current_event();

    if let Some(Event::Key(key)) = current_event {
        match key.code {
            KeyCode::Esc => {
                app.set_cursor_mode(CursorMode::Normal);
            }
            KeyCode::Char('a') => {
                if let CursorMode::Normal = app.cursor_mode() {
                    current_event = None;
                }
                app.set_cursor_mode(CursorMode::Insert);
            }

            _ => {}
        }
    }

    // renders the current screen
    match *app.current_screen() {
        AdminCurrentScreen::Menu => {
            if let Some(Event::Key(key)) = current_event {
                match key.code {
                    KeyCode::Char('q') => {
                        app.set_current_screen(AdminCurrentScreen::Exit);
                    }
                    KeyCode::Down => {
                        if let Some(AdminFocusOn::Line(n)) = app.focus_on() {
                            app.set_focus_on(Some(AdminFocusOn::Line((n + 1) % 3)));
                        }
                    }
                    KeyCode::Up => {
                        if let Some(AdminFocusOn::Line(n)) = app.focus_on() {
                            if *n != 0 {
                                app.set_focus_on(Some(AdminFocusOn::Line((n - 1) % 3)));
                            } else {
                                app.set_focus_on(Some(AdminFocusOn::Line(2)));
                            }
                        }
                    }

                    KeyCode::Enter => {
                        if let Some(AdminFocusOn::Line(n)) = app.focus_on() {
                            match n {
                                0 => {
                                    app.set_db_cursor(0);
                                    app.fetch_users(10);
                                    app.set_current_screen(AdminCurrentScreen::Users);
                                }
                                1 => {
                                    app.set_current_screen(AdminCurrentScreen::Messages);
                                }
                                2 => {
                                    app.set_current_screen(AdminCurrentScreen::Chats);
                                }
                                _ => {}
                            }
                        }
                    }

                    _ => {}
                }
            }

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(2),
                        Constraint::Fill(1),
                        Constraint::Length(2),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            f.render_widget(
                Paragraph::new("Menu")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)),
                chunks[0],
            );

            f.render_stateful_widget(
                List::new(["Users", "Messages", "Chats"])
                    .scroll_padding(3)
                    .highlight_symbol(" >> ")
                    .block(Block::new().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[1],
                &mut ListState::default().with_selected(
                    if let Some(AdminFocusOn::Line(n)) = app.focus_on() {
                        Some(*n)
                    } else {
                        app.set_focus_on(Some(AdminFocusOn::Line(0)));
                        Some(0)
                    },
                ),
            );

            f.render_widget(
                Paragraph::new("Press 'q' to exit")
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .title(app.cursor_mode().as_str()),
                    ),
                chunks[2],
            );
        }
        AdminCurrentScreen::Users => {
            if let Some(Event::Key(key)) = current_event {
                match key.code {
                    KeyCode::Char('q') => {
                        app.set_current_screen(AdminCurrentScreen::Exit);
                    }
                    KeyCode::Down => {
                        if let Some(AdminFocusOn::Line(n)) = app.focus_on() {
                            if *n < app.users().len() - 1 {
                                app.set_focus_on(Some(AdminFocusOn::Line(n + 1)));
                            } else {
                                let idx = app.next_users_page(10);
                                if idx != 0 {
                                    app.set_focus_on(Some(AdminFocusOn::Line(0)));
                                }
                            }
                        }
                    }
                    KeyCode::Up => {
                        if let Some(AdminFocusOn::Line(n)) = app.focus_on() {
                            if *n != 0 {
                                app.set_focus_on(Some(AdminFocusOn::Line(n - 1)));
                            } else {
                                let idx = app.prev_users_page(10);
                                app.set_focus_on(Some(AdminFocusOn::Line(idx - 1)));
                            }
                        }
                    }

                    _ => {}
                }
            }

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(2),
                        Constraint::Fill(1),
                        Constraint::Length(2),
                        Constraint::Length(2),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            f.render_widget(
                Paragraph::new("Users")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)),
                chunks[0],
            );

            let header = ["Id", "Name", "Pwd"]
                .into_iter()
                .map(Cell::from)
                .collect::<Row>()
                .style(Style::default().fg(Color::Yellow))
                .height(1);

            let rows = app.users().iter().map(|data| {
                let id = Cell::from(Text::from(format!("{}", data.id)));
                let username = Cell::from(Text::from(data.username.to_owned()));
                let password = Cell::from(Text::from(data.password.to_owned()));

                [id, username, password].into_iter().collect::<Row>()
            });

            f.render_stateful_widget(
                Table::new(
                    rows,
                    [
                        // + 1 is for padding.
                        Constraint::Length(5),
                        Constraint::Min(30),
                        Constraint::Min(30),
                    ],
                )
                .header(header)
                .highlight_symbol(" >> ")
                .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[1],
                &mut TableState::new().with_selected(
                    if let Some(AdminFocusOn::Line(n)) = app.focus_on() {
                        Some(*n)
                    } else {
                        None
                    },
                ),
            );

            if let Some(msg) = app.error() {
                f.render_widget(
                    Paragraph::new(msg.to_owned())
                        .alignment(Alignment::Center)
                        .style(Style::default().fg(Color::Red))
                        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
                    chunks[2],
                );
            }

            f.render_widget(
                Paragraph::new("Press 'f' to filter | Press 'q' to exit")
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .title(app.cursor_mode().as_str()),
                    ),
                chunks[3],
            );
        }

        AdminCurrentScreen::Messages => todo!(""),
        AdminCurrentScreen::Chats => todo!(""),
        AdminCurrentScreen::Exit => {}
    }
}
