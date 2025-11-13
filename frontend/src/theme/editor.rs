use super::Theme;
use crate::state::VimMode;
use ratzilla::ratatui::style::Style;

pub struct EditorTheme;

impl EditorTheme {
    pub fn border_style(vim_mode: VimMode, is_focused: bool) -> Style {
        if is_focused {
            match vim_mode {
                VimMode::Normal => Style::default().fg(Theme::NORMAL_MODE),
                VimMode::Insert => Style::default().fg(Theme::INSERT_MODE),
            }
        } else {
            Style::default().fg(Theme::OVERLAY1)
        }
    }
}
