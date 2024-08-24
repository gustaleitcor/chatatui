use std::{
    io::Result,
    sync::{Arc, Mutex},
};

use crud_bd::crud::establish_connection;
use diesel::PgConnection;
use ratatui::crossterm::event::{self, Event};

use crate::admin::{AdminCursorMode, AdminFocusOn};

pub enum CurrentScreen {
    Menu,
    Users,
    Messages,
    Chats,
    Exit,
}

pub struct State {
    current_event: Arc<Mutex<Option<Event>>>,
    current_screen: CurrentScreen,
    focus_on: Option<AdminFocusOn>,
    cursor_mode: AdminCursorMode,
    error: Option<Result<String>>,
}

impl State {
    pub fn new() -> State {
        State {
            current_event: Arc::new(Mutex::new(None)),
            current_screen: CurrentScreen::Menu,
            cursor_mode: AdminCursorMode::View('x'),
            focus_on: None,
            error: None,
        }
    }

    pub fn read_event(current_event: &Arc<Mutex<Option<Event>>>) {
        let event = event::read().unwrap();
        current_event.lock().unwrap().replace(event);
    }

    pub fn goto_exit(&mut self) {
        self.current_screen = CurrentScreen::Exit;
    }

    pub fn goto_menu(&mut self) {
        self.current_screen = CurrentScreen::Menu;
    }

    pub fn goto_users(&mut self) {
        self.current_screen = CurrentScreen::Users;
    }

    pub fn goto_messages(&mut self) {
        self.current_screen = CurrentScreen::Messages;
    }

    pub fn goto_chats(&mut self) {
        self.current_screen = CurrentScreen::Chats;
    }

    pub fn has_exited(&self) -> bool {
        matches!(self.current_screen, CurrentScreen::Exit)
    }

    pub fn current_screen(&self) -> &CurrentScreen {
        &self.current_screen
    }

    pub fn take_current_event(&self) -> Option<Event> {
        self.current_event.lock().unwrap().take()
    }

    pub fn get_current_event(&self) -> Option<Event> {
        self.current_event.lock().unwrap().clone()
    }

    pub fn clone_current_event(&self) -> Arc<Mutex<Option<Event>>> {
        self.current_event.clone()
    }

    pub fn toggle_cursor_mode(&mut self) {
        self.cursor_mode = match self.cursor_mode {
            AdminCursorMode::View(_) => AdminCursorMode::Edit('x'),
            AdminCursorMode::Edit(_) => AdminCursorMode::View('x'),
        };
    }

    pub fn set_cursor_mode(&mut self, mode: AdminCursorMode) {
        self.cursor_mode = mode;
    }

    pub fn focus_on(&self) -> &Option<AdminFocusOn> {
        &self.focus_on
    }

    pub fn set_focus_on(&mut self, focus_on: Option<AdminFocusOn>) {
        self.focus_on = focus_on;
    }

    pub fn cursor_mode(&self) -> &AdminCursorMode {
        &self.cursor_mode
    }

    pub fn prompt_message(&self) -> &Option<Result<String>> {
        &self.error
    }

    pub fn set_prompt_message(&mut self, error: Option<Result<String>>) {
        self.error = error;
    }
}
