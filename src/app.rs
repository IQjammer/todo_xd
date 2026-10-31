use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::{action::Action, list::List, ui};

#[derive(Default)]
pub struct App {
    pub exit: bool,
    pub list: List,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        ui::render(self, frame.area(), frame.buffer_mut());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                let action = self.map_key_to_action(key);
                self.update(action);
            }
        } else {
            self.update(Action::Tick);
        }
        Ok(())
    }

    fn map_key_to_action(&self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
            KeyCode::Char('k') => Action::Up,
            KeyCode::Char('j') => Action::Down,
            KeyCode::Char('l') | KeyCode::Char('h') => Action::Done,
            KeyCode::Char('n') => Action::New,
            _ => Action::None,
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.exit = true,
            Action::Up => self.list.up(),
            Action::Down => self.list.down(),
            Action::Done => self.list.done(),
            Action::New => self.list.done(),
            Action::Tick | Action::None => {}
        }
    }
}
