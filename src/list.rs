use ratatui::widgets::ListState;

pub struct TodoItem {
    pub text: String,
    pub completed: bool,
}

pub struct List {
    pub items: Vec<TodoItem>,
    pub list_state: ListState,
    pub input: String,
}

impl Default for List {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            items: vec![
                TodoItem {
                    text: String::from("Hello"),
                    completed: true,
                },
                TodoItem {
                    text: String::from("World!"),
                    completed: true,
                },
            ],
            list_state,
            input: String::new(),
        }
    }
}

impl List {
    pub fn up(&mut self) {
        if !self.items.is_empty() {
            let i = match self.list_state.selected() {
                Some(0) | None => self.items.len() - 1,
                Some(i) => i - 1,
            };
            self.list_state.select(Some(i));
        }
    }

    pub fn down(&mut self) {
        if !self.items.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) if i >= self.items.len() - 1 => 0,
                Some(i) => i + 1,
                None => 0,
            };
            self.list_state.select(Some(i))
        }
    }

    pub fn done(&mut self) {
        if let Some(i) = self.list_state.selected()
            && !self.items.is_empty()
        {
            if self.items[i].completed {
                self.items[i].completed = false;
            } else {
                let mut item = self.items.remove(i);
                item.completed = true;
                self.items.push(item);
            }

            if i >= self.items.len() {
                self.list_state.select(Some(self.items.len() - 1));
            }
        }
    }

    pub fn add(&mut self) {
        let trimmed = self.input.trim();
        if !trimmed.is_empty() {
            let new_item = TodoItem {
                text: trimmed.to_string(),
                completed: false,
            };

            let insert_idx = self
                .items
                .iter()
                .position(|item| item.completed)
                .unwrap_or(self.items.len());

            self.items.insert(insert_idx, new_item);
            self.list_state.select(Some(insert_idx));
            self.input.clear();
        }
    }
}
