use tui_textarea::TextArea;

pub struct EditorState {
    pub textarea: TextArea<'static>,
    pub current_file: Option<String>,
    pub original_content: String,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            textarea: TextArea::default(),
            current_file: None,
            original_content: String::new(),
        }
    }

    pub fn load_content(&mut self, filename: String, content: String) {
        self.current_file = Some(filename);
        self.original_content = content.clone();

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        self.textarea = TextArea::new(lines);
    }

    pub fn get_content(&self) -> String {
        self.textarea.lines().join("\n")
    }
}
