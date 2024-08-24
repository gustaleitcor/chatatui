use std::io::Result;

use ratatui::{
    crossterm::event::{Event, KeyEvent},
    prelude::Backend,
    Frame,
};

use crate::admin::Admin;

pub trait Page<B: Backend> {
    fn render(&self, frame: &mut Frame, app_state: &mut Admin) -> Result<()>;
    fn setup(&mut self, frame: &mut Frame) -> Result<()>;
    fn handle_input(&mut self, key: &KeyEvent, app_state: &mut Admin) -> Result<()>;
    fn handle_resize(&mut self, size: (u16, u16)) -> Result<()>;
    fn cleanup(&mut self) -> Result<()>;

    fn run(&mut self, frame: &mut Frame, app_state: &mut Admin) -> Result<()> {
        self.setup(frame);

        let current_event = app_state.take_current_event();

        if let Some(Event::Key(key)) = current_event {
            self.handle_input(&key, app_state);
        }

        if let Some(Event::Resize(x, y)) = current_event {
            self.handle_resize((x, y))?;
        }

        self.render(frame, app_state)?;

        self.cleanup()?;

        Ok(())
    }
}
