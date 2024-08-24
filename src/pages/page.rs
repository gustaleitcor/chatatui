use std::io::Result;

use ratatui::{
    crossterm::event::{Event, KeyEvent},
    prelude::Backend,
    Frame,
};

use crate::{admin::Admin, state::State};

pub trait Page<B: Backend> {
    fn render(&self, frame: &mut Frame, state: &mut State) -> Result<()>;
    fn setup(&mut self, frame: &mut Frame) -> Result<()>;
    fn handle_input(&mut self, app: &mut Admin, key: &KeyEvent) -> Result<()>;
    fn handle_resize(&mut self, app: &mut Admin, size: (u16, u16)) -> Result<()>;
    fn cleanup(&mut self) -> Result<()>;

    fn run(&mut self, frame: &mut Frame, app: &mut Admin) -> Result<()> {
        self.setup(frame)?;

        let current_event = app.state().take_current_event();

        if let Some(Event::Key(key)) = current_event {
            self.handle_input(app, &key)?;
        }

        if let Some(Event::Resize(x, y)) = current_event {
            self.handle_resize(app, (x, y))?;
        }

        self.render(frame, app.state_mut())?;

        self.cleanup()?;

        Ok(())
    }
}
