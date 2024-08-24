use ratatui::{
    crossterm::event::{Event, KeyCode},
    Frame,
};

use crate::{
    pages::{menu::Menu, page::Page},
    state::CurrentScreen,
};

use crate::admin::Admin;

pub fn ui_admin(frame: &mut Frame, app: &mut Admin, menu: &mut Menu) {
    let app_state = app.state_mut();

    // Does not consume the event because menu should handle it
    let current_event = app_state.get_current_event();
    if let Some(Event::Key(key)) = current_event {
        if key.code == KeyCode::Esc {
            app_state.toggle_cursor_mode();
        }
    }

    // renders the current screen
    match app_state.current_screen() {
        CurrentScreen::Menu => menu.run(frame, app_state).unwrap(),

        CurrentScreen::Users => todo!(""),

        // CurrentScreen::Users => {
        //     let chunks = Layout::default()
        //         .direction(Direction::Vertical)
        //         .constraints(
        //             [
        //                 Constraint::Length(2),
        //                 Constraint::Fill(1),
        //                 Constraint::Length(2),
        //                 Constraint::Length(2),
        //             ]
        //             .as_ref(),
        //         )
        //         .split(f.size());

        //     let n_rows = chunks[1].height.saturating_sub(1) as i64;

        //     if let Some(Event::Resize(_, _)) = current_event {
        //         if let AdminCursorMode::Edit('c') = app.cursor_mode() {
        //             return;
        //         } else {
        //             app.fetch_users(n_rows);
        //         }

        //         if app.users().is_empty() {
        //             app.set_focus_on(None);
        //         } else {
        //             match app.focus_on() {
        //                 Some(AdminFocusOn::Line(n, _)) => {
        //                     if *n >= app.users().len() {
        //                         app.set_focus_on(Some(AdminFocusOn::Line(
        //                             app.users().len() - 1,
        //                             1,
        //                         )));
        //                     } else if app.users().len()
        //                         <= chunks[1].height.saturating_sub(1) as usize
        //                     {
        //                         app.set_db_cursor((app.db_cursor() as u64).saturating_sub(
        //                             1 + (chunks[1].height.saturating_sub(1) as usize
        //                                 - app.users().len())
        //                                 as u64,
        //                         ) as i64);
        //                     }
        //                 }
        //                 _ => {
        //                     app.set_focus_on(Some(AdminFocusOn::Line(0, 1)));
        //                 }
        //             }
        //         }
        //     }

        //     if let Some(Event::Key(key)) = current_event {
        //         match app.cursor_mode() {
        //             AdminCursorMode::View('x') => match key.code {
        //                 KeyCode::Char('q') => {
        //                     app.set_focus_on(Some(AdminFocusOn::Line(0, 1)));
        //                     app.set_current_screen(AdminCurrentScreen::Menu);
        //                 }
        //                 KeyCode::Down => {
        //                     if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
        //                         if !app.users().is_empty() {
        //                             if *n < app.users().len() - 1 {
        //                                 app.set_focus_on(Some(AdminFocusOn::Line(n + 1, 1)));
        //                             } else {
        //                                 app.next_users_page(n_rows);
        //                                 app.set_focus_on(Some(AdminFocusOn::Line(0, 1)));
        //                             }
        //                         } else {
        //                             app.set_focus_on(None);
        //                         }
        //                     }
        //                 }

        //                 KeyCode::Char('f') => {

        //                     // if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
        //                     //     if !app.users().is_empty() {
        //                     //         if *n < app.users().len() - 1 {
        //                     //             app.set_focus_on(Some(AdminFocusOn::Line(n + 1, 1)));
        //                     //         } else {
        //                     //             app.next_users_page(
        //                     //                 chunks[1].height.saturating_sub(1) as i64
        //                     //             );
        //                     //             app.set_focus_on(Some(AdminFocusOn::Line(0, 1)));
        //                     //         }
        //                     //     } else {
        //                     //         app.set_focus_on(None);
        //                     //     }
        //                     // }
        //                 }

        //                 KeyCode::Up => {
        //                     if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
        //                         if !app.users().is_empty() {
        //                             if *n > 0 {
        //                                 app.set_focus_on(Some(AdminFocusOn::Line(n - 1, 1)));
        //                             } else {
        //                                 app.prev_users_page(n_rows);
        //                                 app.set_focus_on(Some(AdminFocusOn::Line(
        //                                     app.users().len() - 1,
        //                                     1,
        //                                 )));
        //                             }
        //                         } else {
        //                             app.set_focus_on(None);
        //                         }
        //                     }
        //                 }

        //                 _ => {}
        //             },

        //             AdminCursorMode::Edit('x') => match key.code {
        //                 KeyCode::Char('d') => {
        //                     if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
        //                         let n = *n;
        //                         if !app.users().is_empty() {
        //                             let user = app.users().get(n).unwrap();
        //                             let user_id = user.id.to_owned();
        //                             match crud_bd::crud::user::delete_user(app.pg_conn(), user_id) {
        //                                 Ok(_) => {
        //                                     app.users_mut().remove(n);
        //                                     app.set_focus_on(Some(AdminFocusOn::Line(
        //                                         n.saturating_sub(1),
        //                                         1,
        //                                     )));
        //                                     app.set_prompt_message(Some(Ok(
        //                                         "User deleted".to_string()
        //                                     )))
        //                                 }

        //                                 Err(err) => {
        //                                     app.set_prompt_message(Some(Err(std::io::Error::new(
        //                                         std::io::ErrorKind::Other,
        //                                         format!("Failed to delete user. {:?}", err),
        //                                     ))))
        //                                 }
        //                             }
        //                         }
        //                     }
        //                 }

        //                 KeyCode::Char('u') => {
        //                     if !app.users().is_empty() {
        //                         if let Some(AdminFocusOn::Line(row, _)) = app.focus_on() {
        //                             if app.users().get(*row).is_some() {
        //                                 app.set_cursor_mode(AdminCursorMode::Edit('u'));
        //                             }
        //                         }
        //                     }
        //                 }

        //                 KeyCode::Char('c') => {
        //                     app.set_cursor_mode(AdminCursorMode::Edit('c'));
        //                     app.users_mut().push(User {
        //                         id: -1,
        //                         username: "".to_string(),
        //                         password: "".to_string(),
        //                     });
        //                     app.set_focus_on(Some(AdminFocusOn::Line(app.users().len() - 1, 1)));
        //                 }

        //                 KeyCode::Down => {
        //                     if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
        //                         if !app.users().is_empty() {
        //                             if *n < app.users().len() - 1 {
        //                                 app.set_focus_on(Some(AdminFocusOn::Line(n + 1, 1)));
        //                             } else {
        //                                 app.next_users_page(n_rows);
        //                                 app.set_focus_on(Some(AdminFocusOn::Line(0, 1)));
        //                             }
        //                         } else {
        //                             app.set_focus_on(None);
        //                         }
        //                     }
        //                 }

        //                 KeyCode::Up => {
        //                     if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
        //                         if !app.users().is_empty() {
        //                             if *n > 0 {
        //                                 app.set_focus_on(Some(AdminFocusOn::Line(n - 1, 1)));
        //                             } else {
        //                                 app.prev_users_page(n_rows);
        //                                 app.set_focus_on(Some(AdminFocusOn::Line(
        //                                     app.users().len() - 1,
        //                                     1,
        //                                 )));
        //                             }
        //                         } else {
        //                             app.set_focus_on(None);
        //                         }
        //                     }
        //                 }

        //                 _ => {}
        //             },

        //             AdminCursorMode::Edit('u') => {
        //                 if let Some(AdminFocusOn::Line(row, col)) = app.focus_on().clone() {
        //                     if let Some(Event::Key(key)) = current_event {
        //                         match key.code {
        //                             KeyCode::Enter => {
        //                                 if let Some(user) = app.users_mut().get_mut(row) {
        //                                     let user_id = &user.id.to_owned();
        //                                     let user_name = &user.username.to_owned();
        //                                     let user_password = &user.password.to_owned();

        //                                     match crud_bd::crud::user::update_user_username(
        //                                         app.pg_conn(),
        //                                         *user_id,
        //                                         user_name,
        //                                     ) {
        //                                         Ok(_) => app.set_prompt_message(Some(Ok(
        //                                             "User created".to_string(),
        //                                         ))),

        //                                         Err(err) => {
        //                                             app.fetch_users(n_rows);
        //                                             app.set_prompt_message(Some(Err(
        //                                                 std::io::Error::new(
        //                                                     std::io::ErrorKind::Other,
        //                                                     format!(
        //                                                     "Failed to create user username. {:?}",
        //                                                     err.to_string()
        //                                                 ),
        //                                                 ),
        //                                             )));
        //                                             return;
        //                                         }
        //                                     }

        //                                     match crud_bd::crud::user::update_user_password(
        //                                         app.pg_conn(),
        //                                         *user_id,
        //                                         user_password.as_str(),
        //                                     ) {
        //                                         Ok(_) => app.set_prompt_message(Some(Ok(
        //                                             "User updated".to_string(),
        //                                         ))),

        //                                         Err(err) => {
        //                                             app.fetch_users(n_rows);

        //                                             app.set_prompt_message(Some(Err(
        //                                                 std::io::Error::new(
        //                                                     std::io::ErrorKind::Other,
        //                                                     format!(
        //                                                     "Failed to update user password. {:?}",
        //                                                     err,
        //                                                 ),
        //                                                 ),
        //                                             )));
        //                                             return;
        //                                         }
        //                                     }
        //                                 }

        //                                 app.toggle_cursor_mode();
        //                                 app.set_focus_on(Some(AdminFocusOn::Line(row, 1)));
        //                             }

        //                             KeyCode::Char(c) => {
        //                                 if let Some(user) = app.users_mut().get_mut(row) {
        //                                     match col {
        //                                         1 => {
        //                                             user.username.push(c);
        //                                         }

        //                                         2 => {
        //                                             user.password.push(c);
        //                                         }

        //                                         _ => {}
        //                                     }
        //                                 }
        //                             }

        //                             KeyCode::Backspace => {
        //                                 if let Some(user) = app.users_mut().get_mut(row) {
        //                                     match col {
        //                                         1 => {
        //                                             user.username.pop();
        //                                         }

        //                                         2 => {
        //                                             user.password.pop();
        //                                         }

        //                                         _ => (),
        //                                     }
        //                                 }
        //                             }

        //                             KeyCode::Left => {
        //                                 if col > 1 {
        //                                     app.set_focus_on(Some(AdminFocusOn::Line(
        //                                         row,
        //                                         col - 1,
        //                                     )));
        //                                 }
        //                             }

        //                             KeyCode::Right => {
        //                                 if col < 2 {
        //                                     app.set_focus_on(Some(AdminFocusOn::Line(
        //                                         row,
        //                                         col + 1,
        //                                     )));
        //                                 }
        //                             }

        //                             _ => (),
        //                         }
        //                     }
        //                 }
        //             }

        //             AdminCursorMode::Edit('c') => {
        //                 if let Some(AdminFocusOn::Line(row, col)) = app.focus_on().clone() {
        //                     if let Some(Event::Key(key)) = current_event {
        //                         match key.code {
        //                             KeyCode::Enter => {
        //                                 let username = app.new_user().username.clone();
        //                                 let password = app.new_user().password.clone();

        //                                 match crud_bd::crud::user::create_user(
        //                                     app.pg_conn(),
        //                                     username.as_str(),
        //                                     password.as_str(),
        //                                 ) {
        //                                     Ok(_) => {
        //                                         app.set_prompt_message(Some(Ok(
        //                                             "User created".to_string()
        //                                         )));
        //                                     }

        //                                     Err(err) => app.set_prompt_message(Some(Err(
        //                                         std::io::Error::new(
        //                                             std::io::ErrorKind::Other,
        //                                             format!("Failed to create user. {:?}", err),
        //                                         ),
        //                                     ))),
        //                                 }

        //                                 app.toggle_cursor_mode();
        //                                 app.set_focus_on(Some(AdminFocusOn::Line(0, 1)));
        //                                 app.fetch_users(n_rows);
        //                                 app.new_user_mut().username.clear();
        //                                 app.new_user_mut().password.clear();
        //                             }

        //                             KeyCode::Char(c) => match col {
        //                                 1 => {
        //                                     app.new_user_mut().username.push(c);
        //                                 }

        //                                 2 => {
        //                                     app.new_user_mut().password.push(c);
        //                                 }

        //                                 _ => {}
        //                             },

        //                             KeyCode::Backspace => match col {
        //                                 1 => {
        //                                     app.new_user_mut().username.pop();
        //                                 }

        //                                 2 => {
        //                                     app.new_user_mut().password.pop();
        //                                 }

        //                                 _ => (),
        //                             },

        //                             KeyCode::Left => {
        //                                 if col > 1 {
        //                                     app.set_focus_on(Some(AdminFocusOn::Line(
        //                                         row,
        //                                         col - 1,
        //                                     )));
        //                                 }
        //                             }

        //                             KeyCode::Right => {
        //                                 if col < 2 {
        //                                     app.set_focus_on(Some(AdminFocusOn::Line(
        //                                         row,
        //                                         col + 1,
        //                                     )));
        //                                 }
        //                             }

        //                             _ => (),
        //                         }
        //                     }
        //                 }
        //             }

        //             _ => {}
        //         }
        //     }

        //     let line = match app.focus_on() {
        //         Some(AdminFocusOn::Line(n, _)) => *n as i32,
        //         _ => -1,
        //     };

        //     f.render_widget(
        //         Paragraph::new("Users").alignment(Alignment::Center).block(
        //             Block::default()
        //                 .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        //                 .title(format!(
        //                     "db_cursor: {} | Page: {} | Userslen: {} | Line: {} | TableHeight: {}",
        //                     app.db_cursor(),
        //                     app.db_cursor()
        //                         .checked_div(chunks[1].as_size().height.saturating_sub(1) as i64)
        //                         .unwrap_or(0),
        //                     app.users().len(),
        //                     line,
        //                     chunks[1].as_size().height
        //                 )),
        //         ),
        //         chunks[0],
        //     );

        //     let header = ["Id", "Name", "Pwd"]
        //         .into_iter()
        //         .map(Cell::from)
        //         .collect::<Row>()
        //         .style(Style::default().fg(Color::Yellow))
        //         .height(1);

        //     let rows = app.users().iter().enumerate().map(|(i, data)| {
        //         if let AdminCursorMode::Edit('u') = app.cursor_mode() {
        //             if let Some(AdminFocusOn::Line(row, col)) = app.focus_on() {
        //                 if i == *row {
        //                     let mut cells = vec![];
        //                     for (j, cell) in [
        //                         data.id.to_string(),
        //                         data.username.to_owned(),
        //                         data.password.to_owned(),
        //                     ]
        //                     .iter()
        //                     .enumerate()
        //                     {
        //                         if j == *col {
        //                             cells.push(
        //                                 Cell::from(Text::from(cell.to_owned()))
        //                                     .style(Style::default().bg(Color::LightBlue)),
        //                             );
        //                         } else {
        //                             cells.push(Cell::from(Text::from(cell.to_owned())));
        //                         }
        //                     }
        //                     return Row::new(cells);
        //                 }
        //             }
        //         }

        //         if let AdminCursorMode::Edit('c') = app.cursor_mode() {
        //             if let Some(AdminFocusOn::Line(row, col)) = app.focus_on() {
        //                 if i == *row {
        //                     let mut cells = vec![];
        //                     for (j, cell) in [
        //                         "New User:".to_string(),
        //                         app.new_user().username.to_owned(),
        //                         app.new_user().password.to_owned(),
        //                     ]
        //                     .iter()
        //                     .enumerate()
        //                     {
        //                         if j == *col {
        //                             cells.push(
        //                                 Cell::from(Text::from(cell.to_owned()))
        //                                     .style(Style::default().bg(Color::LightBlue)),
        //                             );
        //                         } else {
        //                             cells.push(Cell::from(Text::from(cell.to_owned())));
        //                         }
        //                     }
        //                     return Row::new(cells);
        //                 }
        //             }
        //         }

        //         let id = Cell::from(Text::from(format!("{}", data.id)));
        //         let username = Cell::from(Text::from(data.username.to_owned()));
        //         let password = Cell::from(Text::from(data.password.to_owned()));

        //         [id, username, password].into_iter().collect::<Row>()
        //     });

        //     f.render_stateful_widget(
        //         Table::new(
        //             rows,
        //             [
        //                 // + 1 is for padding.
        //                 Constraint::Length(9),
        //                 Constraint::Max(32),
        //                 Constraint::Max(32),
        //             ],
        //         )
        //         .header(header)
        //         .highlight_symbol(" >> ")
        //         .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
        //         chunks[1],
        //         &mut TableState::new().with_selected(
        //             if let Some(AdminFocusOn::Line(n, _)) = app.focus_on() {
        //                 Some(*n)
        //             } else {
        //                 Some(0)
        //             },
        //         ),
        //     );

        //     if let Some(Ok(msg)) = app.prompt_message() {
        //         f.render_widget(
        //             Paragraph::new(msg.to_owned())
        //                 .alignment(Alignment::Center)
        //                 .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
        //             chunks[2],
        //         );
        //     } else if let Some(Err(msg)) = app.prompt_message() {
        //         f.render_widget(
        //             Paragraph::new(msg.to_string())
        //                 .alignment(Alignment::Center)
        //                 .style(Style::default().fg(Color::Red))
        //                 .block(
        //                     Block::default()
        //                         .borders(Borders::LEFT | Borders::RIGHT)
        //                         .style(Style::default().fg(Color::White)),
        //                 ),
        //             chunks[2],
        //         );
        //     } else {
        //         f.render_widget(
        //             Block::default().borders(Borders::LEFT | Borders::RIGHT),
        //             chunks[2],
        //         );
        //     };

        //     match app.cursor_mode() {
        //         AdminCursorMode::View('f') => {
        //             f.render_widget(
        //                 Paragraph::new("Press 'f' to filter | Press 'q' to goto menu")
        //                     .alignment(Alignment::Center)
        //                     .block(
        //                         Block::default()
        //                             .borders(Borders::TOP)
        //                             .title(app.cursor_mode().as_str()),
        //                     ),
        //                 chunks[3],
        //             );
        //         }
        //         AdminCursorMode::View(_) => {
        //             f.render_widget(
        //                 Paragraph::new("Press 'f' to filter | Press 'q' to goto menu")
        //                     .alignment(Alignment::Center)
        //                     .block(
        //                         Block::default()
        //                             .borders(Borders::TOP)
        //                             .title(app.cursor_mode().as_str()),
        //                     ),
        //                 chunks[3],
        //             );
        //         }

        //         AdminCursorMode::Edit(_) => {
        //             f.render_widget(
        //                 Paragraph::new(
        //                     "Press 'c' to create | Press 'd' to delete | Press 'u' to update | Press 'esc' to cancel",
        //                 )
        //                 .alignment(Alignment::Center)
        //                 .block(
        //                     Block::default()
        //                         .borders(Borders::TOP)
        //                         .title(app.cursor_mode().as_str()),
        //                 ),
        //                 chunks[3],
        //             );
        //         }
        //     }
        // }
        CurrentScreen::Messages => todo!(""),
        CurrentScreen::Chats => todo!(""),
        CurrentScreen::Exit => {}
    }
}
