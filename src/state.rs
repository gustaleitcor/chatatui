use std::{
    io::Result,
    sync::{Arc, Mutex},
    time::Instant,
};

use crud_bd::crud::user;
use ratatui::crossterm::event::{self, Event};

use crate::app::{CursorMode, FocusOn};

pub enum CurrentScreen {
    Login,
    Register,
    Menu,
    Users,
    Messages,
    AdminChats,
    ClientChats,
    Chat,
    Exit,
}

pub struct State {
    user: user::User,
    current_event: Arc<Mutex<Option<Event>>>,
    current_screen: CurrentScreen,
    focus_on: Option<FocusOn>,
    cursor_mode: CursorMode,
    error: Option<Result<String>>,
    error_timestamp: Option<Instant>,
    screen_has_changed: bool,
}

impl State {
    pub fn new() -> State {
        State {
            user: user::User {
                id: 0,
                username: String::new(),
                password: String::new(),
            },
            current_event: Arc::new(Mutex::new(None)),
            current_screen: CurrentScreen::Login,
            cursor_mode: CursorMode::View('x'),
            focus_on: None,
            error: None,
            screen_has_changed: true,
            error_timestamp: None,
        }
    }

    pub fn read_event(current_event: &Arc<Mutex<Option<Event>>>) {
        let event = event::read().unwrap();
        current_event.lock().unwrap().replace(event);
    }

    pub fn goto_exit(&mut self) {
        self.current_screen = CurrentScreen::Exit;
        self.screen_has_changed = true;
    }

    pub fn goto_menu(&mut self) {
        self.current_screen = CurrentScreen::Menu;
        self.screen_has_changed = true;
    }

    pub fn goto_users(&mut self) {
        self.current_screen = CurrentScreen::Users;
        self.screen_has_changed = true;
    }

    pub fn goto_messages(&mut self) {
        self.current_screen = CurrentScreen::Messages;
        self.screen_has_changed = true;
    }

    pub fn goto_admin_chats(&mut self) {
        self.current_screen = CurrentScreen::AdminChats;
        self.screen_has_changed = true;
    }

    pub fn goto_login(&mut self) {
        self.current_screen = CurrentScreen::Login;
        self.screen_has_changed = true;
    }

    pub fn goto_register(&mut self) {
        self.current_screen = CurrentScreen::Register;
        self.screen_has_changed = true;
    }

    pub fn goto_chat(&mut self) {
        self.current_screen = CurrentScreen::Chat;
        self.screen_has_changed = true;
    }

    pub fn goto_client_chats(&mut self) {
        self.current_screen = CurrentScreen::ClientChats;
        self.screen_has_changed = true;
    }

    pub fn has_screen_changed(&self) -> bool {
        self.screen_has_changed
    }

    pub fn set_screen_has_changed(&mut self, value: bool) {
        self.screen_has_changed = value;
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
            CursorMode::View(_) => CursorMode::Edit('x'),
            CursorMode::Edit(_) => CursorMode::View('x'),
        };
    }

    pub fn set_cursor_mode(&mut self, mode: CursorMode) {
        self.cursor_mode = mode;
    }

    pub fn focus_on(&self) -> &Option<FocusOn> {
        &self.focus_on
    }

    pub fn set_focus_on(&mut self, focus_on: Option<FocusOn>) {
        self.focus_on = focus_on;
    }

    pub fn cursor_mode(&self) -> &CursorMode {
        &self.cursor_mode
    }

    pub fn prompt_message(&self) -> &Option<Result<String>> {
        &self.error
    }

    pub fn set_prompt_message(&mut self, error: Option<Result<String>>) {
        self.error_timestamp = Some(Instant::now());
        self.error = error;
    }

    pub fn clear_prompt_message(&mut self) {
        self.error = None;
    }

    pub fn error_timestamp(&self) -> &Option<Instant> {
        &self.error_timestamp
    }

    pub fn set_user(&mut self, user: user::User) {
        self.user = user;
    }

    pub fn user(&self) -> &user::User {
        &self.user
    }
}
