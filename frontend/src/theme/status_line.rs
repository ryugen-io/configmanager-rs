use super::Theme;
use crate::state::VimMode;
use ratzilla::ratatui::style::{Modifier, Style};

/// Theme styles for the status line widget
///
/// This component follows the standard theme pattern:
/// - Uses standard background style
/// - Uses standard label/value styles
/// - Uses semantic colors for status messages (success/error)
/// - Provides Vim mode specific styling
pub struct StatusLineTheme;

impl StatusLineTheme {
    /// Background style for status line
    pub fn background() -> Style {
        Theme::standard_background()
    }

    /// Text label for current Vim mode
    pub fn mode_text(vim_mode: VimMode) -> &'static str {
        match vim_mode {
            VimMode::Normal => "NORMAL",
            VimMode::Insert => "INSERT",
        }
    }

    /// Style for Vim mode indicator
    pub fn mode_style(vim_mode: VimMode) -> Style {
        let color = match vim_mode {
            VimMode::Normal => Theme::NORMAL_MODE,
            VimMode::Insert => Theme::INSERT_MODE,
        };
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    }

    /// Style for filename display
    pub fn filename_style() -> Style {
        Theme::standard_normal_item()
    }

    /// Style for modified indicator ([+])
    pub fn modified_style() -> Style {
        Style::default().fg(Theme::MODIFIED)
    }

    /// Style for "no file loaded" message
    pub fn no_file_style() -> Style {
        Theme::standard_label()
    }

    /// Style for success/info messages
    pub fn status_message_style() -> Style {
        Style::default().fg(Theme::SUCCESS)
    }

    /// Style for error messages
    pub fn error_message_style() -> Style {
        Style::default().fg(Theme::ERROR)
    }

    /// Style for help text
    pub fn help_text_style() -> Style {
        Theme::standard_label()
    }

    /// Style for field labels
    pub fn label_style() -> Style {
        Theme::standard_label()
    }

    /// Style for field values
    pub fn value_style() -> Style {
        Theme::standard_value()
    }
}
