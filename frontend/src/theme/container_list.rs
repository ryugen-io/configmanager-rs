use super::Theme;
use ratzilla::ratatui::style::{Color, Modifier, Style};

/// Theme styles for the container list widget
///
/// This component follows the standard theme pattern with some custom additions:
/// - Uses standard border styles for focus states
/// - Uses standard highlight for selected items
/// - Adds semantic colors for container states
pub struct ContainerListTheme;

impl ContainerListTheme {
    /// Style for container IDs
    pub fn id_style() -> Style {
        Theme::standard_value()
    }

    /// Style for container names
    pub fn name_style() -> Style {
        Theme::standard_normal_item()
    }

    /// Style for status information text
    pub fn status_info_style() -> Style {
        Theme::standard_label()
    }

    /// Color for container state badge (semantic)
    pub fn status_color(state: &str) -> Color {
        match state {
            "running" => Theme::SUCCESS,
            "exited" => Theme::OVERLAY1,
            _ => Theme::YELLOW,
        }
    }

    /// Border style when container list is focused
    pub fn border_focused() -> Style {
        Theme::standard_border_focused()
    }

    /// Border style when container list is not focused
    pub fn border_unfocused() -> Style {
        Theme::standard_border_unfocused()
    }

    /// Style for selected/highlighted container
    pub fn highlight_style() -> Style {
        Theme::standard_highlight_bg()
            .fg(Theme::TEXT)
            .add_modifier(Modifier::BOLD)
    }
}
