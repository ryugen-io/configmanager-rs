use super::{ContainerListState, EditorState, FileListState, MenuState};
use crate::storage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Menu,
    FileList,
    Editor,
    ContainerList,
}

impl Pane {
    pub fn as_str(&self) -> &'static str {
        match self {
            Pane::Menu => "Menu",
            Pane::FileList => "FileList",
            Pane::Editor => "Editor",
            Pane::ContainerList => "ContainerList",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Menu" => Some(Pane::Menu),
            "FileList" => Some(Pane::FileList),
            "Editor" => Some(Pane::Editor),
            "ContainerList" => Some(Pane::ContainerList),
            _ => None,
        }
    }
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
    pub container_list: ContainerListState,
    pub editor: EditorState,
    pub dirty: bool,
    pub status_message: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        let mut state = Self {
            focus: Pane::Menu,
            vim_mode: VimMode::Normal,
            menu: MenuState::new(),
            file_list: FileListState::new(),
            container_list: ContainerListState::new(),
            editor: EditorState::new(),
            dirty: false,
            status_message: None,
        };

        // Try to restore from localStorage
        if let Some(saved) = storage::load_state()
            && let Some(pane) = Pane::from_str(&saved.pane)
        {
            state.focus = pane;

            // If we were in the editor, restore the file
            if pane == Pane::Editor
                && let (Some(filename), Some(content)) = (saved.filename, saved.content)
            {
                state.editor.load_content(filename, content);
                state.dirty = false;
            }
        }

        state
    }

    pub fn save_to_storage(&self) {
        let filename = self.editor.current_file.as_deref();
        let content = if self.editor.current_file.is_some() {
            Some(self.editor.textarea.lines().join("\n"))
        } else {
            None
        };

        storage::save_state(self.focus.as_str(), filename, content.as_deref());
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
