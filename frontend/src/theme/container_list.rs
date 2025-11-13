use super::Theme;
use ratzilla::ratatui::style::{Modifier, Style};

pub struct ContainerListTheme;

impl ContainerListTheme {
    pub fn id_style() -> Style {
        Style::default().fg(Theme::YELLOW)
    }

    pub fn name_style() -> Style {
        Style::default().fg(Theme::TEXT)
    }

    pub fn status_info_style() -> Style {
        Style::default().fg(Theme::SUBTEXT0)
    }

    pub fn status_color(state: &str) -> ratzilla::ratatui::style::Color {
        match state {
            "running" => Theme::GREEN,
            "exited" => Theme::OVERLAY1,
            _ => Theme::YELLOW,
        }
    }

    pub fn border_focused() -> Style {
        Style::default().fg(Theme::ACCENT)
    }

    pub fn border_unfocused() -> Style {
        Style::default().fg(Theme::OVERLAY1)
    }

    pub fn highlight_style() -> Style {
        Style::default()
            .bg(Theme::SURFACE1)
            .fg(Theme::TEXT)
            .add_modifier(Modifier::BOLD)
    }
}
