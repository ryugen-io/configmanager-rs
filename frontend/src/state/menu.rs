pub struct MenuState {
    pub items: Vec<String>,
    pub selected_index: usize,
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            items: vec!["Config Files".to_string(), "Container".to_string()],
            selected_index: 0,
        }
    }

    pub fn next(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.items.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn selected(&self) -> Option<&String> {
        self.items.get(self.selected_index)
    }
}
