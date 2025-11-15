use super::{Theme, SELECTED_PREFIX};
use ratzilla::ratatui::style::Style;

/// Theme styles for the file list widget
///
/// This component follows the standard theme pattern:
/// - Uses standard border styles for focus states
/// - Uses standard item styles for selection
/// - Uses common prefix constants
pub struct FileListTheme;

impl FileListTheme {
    /// Border style when file list is focused
    pub fn border_focused() -> Style {
        Theme::standard_border_focused()
    }

    /// Border style when file list is not focused
    pub fn border_unfocused() -> Style {
        Theme::standard_border_unfocused()
    }

    /// Style for selected file in the list
    pub fn selected_item_style() -> Style {
        Theme::standard_selected_item()
    }

    /// Style for normal (unselected) files
    pub fn normal_item_style() -> Style {
        Theme::standard_normal_item()
    }

    /// Prefix shown before selected items
    pub fn selected_prefix() -> &'static str {
        SELECTED_PREFIX
    }
}
