use super::{EditorState, FileListState, MenuState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Menu,
    FileList,
    Editor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VimMode {
    Normal,
    Insert,
}

pub struct AppState {
    pub focus: Pane,
    pub vim_mode: VimMode,
    pub menu: MenuState,
    pub file_list: FileListState,
    pub editor: EditorState,
    pub dirty: bool,
    pub status_message: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            focus: Pane::Menu,
            vim_mode: VimMode::Normal,
            menu: MenuState::new(),
            file_list: FileListState::new(),
            editor: EditorState::new(),
            dirty: false,
            status_message: None,
        }
    }

    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
    }

    #[allow(dead_code)]
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    pub fn check_dirty(&mut self) {
        let current_content = self.editor.textarea.lines().join("\n");
        self.dirty = current_content != self.editor.original_content;
    }
}
