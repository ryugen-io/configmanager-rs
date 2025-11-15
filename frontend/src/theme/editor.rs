use super::Theme;
use crate::state::VimMode;
use ratzilla::ratatui::style::Style;

/// Theme styles for the text editor widget
///
/// This component follows the standard theme pattern with special handling:
/// - Border color changes based on Vim mode when focused
/// - Uses NORMAL_MODE color for Normal mode (blue)
/// - Uses INSERT_MODE color for Insert mode (purple)
/// - Uses standard unfocused style when not focused
pub struct EditorTheme;

impl EditorTheme {
    /// Border style for editor, reflects Vim mode when focused
    ///
    /// # Arguments
    /// * `vim_mode` - Current Vim mode (Normal or Insert)
    /// * `is_focused` - Whether the editor has focus
    pub fn border_style(vim_mode: VimMode, is_focused: bool) -> Style {
        if is_focused {
            match vim_mode {
                VimMode::Normal => Style::default().fg(Theme::NORMAL_MODE),
                VimMode::Insert => Style::default().fg(Theme::INSERT_MODE),
            }
        } else {
            Theme::standard_border_unfocused()
        }
    }
}
