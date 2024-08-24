use std::io::Result;

use ratatui::{
    crossterm::event::{Event, KeyEvent},
    prelude::Backend,
    Frame,
};

use crate::state::State;

pub trait Page<B: Backend> {
    fn render(&self, frame: &mut Frame, state: &mut State) -> Result<()>;
    fn setup(&mut self, frame: &mut Frame) -> Result<()>;
    fn handle_input(&mut self, key: &KeyEvent, state: &mut State) -> Result<()>;
    fn handle_resize(&mut self, size: (u16, u16)) -> Result<()>;
    fn cleanup(&mut self) -> Result<()>;

    fn run(&mut self, frame: &mut Frame, state: &mut State) -> Result<()> {
        self.setup(frame)?;

        let current_event = state.take_current_event();

        if let Some(Event::Key(key)) = current_event {
            self.handle_input(&key, state)?;
        }

        if let Some(Event::Resize(x, y)) = current_event {
            self.handle_resize((x, y))?;
        }

        self.render(frame, state)?;

        self.cleanup()?;

        Ok(())
    }
}
