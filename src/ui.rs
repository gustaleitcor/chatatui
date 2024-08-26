use ratatui::{
    crossterm::event::{Event, KeyCode},
    Frame,
};

use crate::{
    app::{CursorMode, FocusOn},
    pages::{chats::Chats, menu::Menu, messages::Messages, page::Page, users::Users},
    state::CurrentScreen,
};

use crate::app::App;

pub fn ui(
    frame: &mut Frame,
    app: &mut App,
    menu: &mut Menu,
    users: &mut Users,
    messages: &mut Messages,
    chats: &mut Chats,
) {
    let app_state = app.state_mut();

    // Does not consume the event because menu should handle it
    let current_event = app_state.get_current_event();
    if let Some(Event::Key(key)) = current_event {
        if key.code == KeyCode::Esc {
            if let Some(FocusOn::Filter(_)) = app_state.focus_on() {
                app_state.set_focus_on(Some(FocusOn::Line(0, 1)));
                app_state.set_cursor_mode(CursorMode::View('x'));
            } else {
                app_state.toggle_cursor_mode();
            }
        }
    }

    // renders the current screen
    match app_state.current_screen() {
        CurrentScreen::Menu => menu.run(frame, app).unwrap(),
        CurrentScreen::Users => users.run(frame, app).unwrap(),
        CurrentScreen::Messages => messages.run(frame, app).unwrap(),
        CurrentScreen::Chats => chats.run(frame, app).unwrap(),
        CurrentScreen::Exit => {}
    }
}
