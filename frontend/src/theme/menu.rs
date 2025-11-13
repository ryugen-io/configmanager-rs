use super::Theme;
use ratzilla::ratatui::style::{Modifier, Style};

pub struct MenuTheme;

impl MenuTheme {
    pub fn title_style() -> Style {
        Style::default()
            .fg(Theme::ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn border_style() -> Style {
        Style::default().fg(Theme::ACCENT)
    }

    pub fn selected_item_style() -> Style {
        Style::default()
            .fg(Theme::SELECTED)
            .add_modifier(Modifier::BOLD)
    }

    pub fn normal_item_style() -> Style {
        Style::default().fg(Theme::TEXT)
    }

    pub fn selected_prefix() -> &'static str {
        "> "
    }

    pub fn normal_prefix() -> &'static str {
        "  "
    }
}
