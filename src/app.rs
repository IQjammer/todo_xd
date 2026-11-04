use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::{action::Action, list::List, ui};

#[derive(Default, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Default)]
pub struct App {
    pub exit: bool,
    pub list: List,
    pub mode: InputMode,
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
        match self.mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
                KeyCode::Char('k') => Action::Up,
                KeyCode::Char('j') => Action::Down,
                KeyCode::Char('l') | KeyCode::Char('h') => Action::Done,
                KeyCode::Char('r') => Action::Delete,
                KeyCode::Char('n') => Action::EnterEditingMode,
                _ => Action::None,
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => Action::SubmitInput,
                KeyCode::Esc => Action::EnterNormalMode,
                KeyCode::Backspace => Action::DeleteChar,
                KeyCode::Char(c) => Action::InsertChar(c),
                _ => Action::None,
            },
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.exit = true,
            Action::Up => self.list.up(),
            Action::Down => self.list.down(),
            Action::Done => self.list.done(),
            Action::Delete => self.list.delete(),

            Action::EnterEditingMode => self.mode = InputMode::Editing,
            Action::EnterNormalMode => {
                self.list.input.clear();
                self.mode = InputMode::Normal;
            }
            Action::InsertChar(c) => self.list.input.push(c),
            Action::DeleteChar => {
                self.list.input.pop();
            }
            Action::SubmitInput => {
                self.list.add();
                self.mode = InputMode::Normal;
            }

            Action::Tick | Action::None => {}
        }
    }
}
