use std::{
    io::Result,
    sync::{Arc, Mutex},
};

use diesel::PgConnection;
use ratatui::crossterm::event::Event;

use crate::admin::{AdminCurrentScreen, AdminCursorMode, AdminFocusOn};

struct State {
    current_event: Arc<Mutex<Option<Event>>>,
    current_screen: AdminCurrentScreen,
    focus_on: Option<AdminFocusOn>,
    cursor_mode: AdminCursorMode,
    error: Option<Result<String>>,
    pg_conn: PgConnection,
}
