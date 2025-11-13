pub struct FileListState {
    pub files: Vec<String>,
    pub selected_index: usize,
}

impl FileListState {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn next(&mut self) {
        if !self.files.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.files.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.files.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.files.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn selected(&self) -> Option<&String> {
        self.files.get(self.selected_index)
    }
}
