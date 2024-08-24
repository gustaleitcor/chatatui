use ratatui::{
    crossterm::event::{Event, KeyCode},
    Frame,
};

use crate::{
    pages::{menu::Menu, page::Page, users::Users},
    state::CurrentScreen,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &mut App, menu: &mut Menu, users: &mut Users) {
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
        CurrentScreen::Menu => menu.run(frame, app).unwrap(),
        CurrentScreen::Users => users.run(frame, app).unwrap(),
        CurrentScreen::Messages => todo!(""),
        CurrentScreen::Chats => todo!(""),
        CurrentScreen::Exit => {}
    }
}
