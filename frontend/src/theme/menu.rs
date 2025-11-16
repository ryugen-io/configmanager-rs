use super::{NORMAL_PREFIX, SELECTED_PREFIX, Theme};
use ratzilla::ratatui::style::Style;

/// Theme styles for the main menu widget
///
/// This component follows the standard theme pattern:
/// - Uses standard border and title styles
/// - Uses standard item styles for selection
/// - Uses common prefix constants
pub struct MenuTheme;

impl MenuTheme {
    /// Style for menu title
    pub fn title_style() -> Style {
        Theme::standard_title()
    }

    /// Border style for menu (always focused when visible)
    pub fn border_style() -> Style {
        Theme::standard_border_focused()
    }

    /// Style for selected menu item
    pub fn selected_item_style() -> Style {
        Theme::standard_selected_item()
    }

    /// Style for normal (unselected) menu items
    pub fn normal_item_style() -> Style {
        Theme::standard_normal_item()
    }

    /// Prefix shown before selected menu items
    pub fn selected_prefix() -> &'static str {
        SELECTED_PREFIX
    }

    /// Prefix shown before normal menu items
    pub fn normal_prefix() -> &'static str {
        NORMAL_PREFIX
    }
}
