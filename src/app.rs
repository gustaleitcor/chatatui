use crate::ui::ui;
use std::{
    io::{stdout, Result},
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::Backend,
    Terminal,
};

pub enum CurrentScreen {
    Register,
    Login,
    Chat,
    Exit,
}

pub struct App {
    current_event: Arc<Mutex<Option<Event>>>,
    current_screen: CurrentScreen,
    login: String,
    password: String,
    pub messages: Vec<String>,
    pub message: String,
}

impl App {
    pub fn new() -> App {
        App {
            current_event: Arc::new(Mutex::new(None)),
            current_screen: CurrentScreen::Login,
            login: String::new(),
            password: String::new(),
            messages: Vec::new(),
            message: String::new(),
        }
    }

    pub fn setup(&self) -> Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        stdout().execute(EnableMouseCapture)?;
        Ok(())
    }

    pub fn cleanup(&self) -> Result<()> {
        stdout().execute(DisableMouseCapture)?;
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        thread::spawn({
            let current_event = self.current_event.clone();
            move || loop {
                App::read_event(current_event.clone());
            }
        });
        loop {
            let now = std::time::Instant::now();

            terminal.draw(|f| {
                ui(f, self);
            })?;

            if let CurrentScreen::Exit = self.current_screen {
                break;
            }

            sleep(
                Duration::from_millis(16)
                    .checked_sub(now.elapsed())
                    .unwrap_or(Duration::from_millis(0)),
            );
        }

        Ok(())
    }

    fn read_event(current_event: Arc<Mutex<Option<Event>>>) {
        let event = event::read().unwrap();
        current_event.lock().unwrap().replace(event);
    }

    pub fn current_screen(&self) -> &CurrentScreen {
        &self.current_screen
    }

    pub fn set_current_screen(&mut self, screen: CurrentScreen) {
        self.current_screen = screen;
    }

    pub fn get_current_event(&self) -> Option<Event> {
        self.current_event.lock().unwrap().take()
    }
}
