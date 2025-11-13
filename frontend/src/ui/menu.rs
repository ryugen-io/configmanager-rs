use crate::{state::AppState, theme::Theme};
use ratzilla::ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    // Center the menu vertically and horizontally
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(15),
            Constraint::Percentage(30),
        ])
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(vertical_chunks[1]);

    let menu_area = horizontal_chunks[1];

    // Build menu content
    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "CONFIG MANAGER",
            Style::default()
                .fg(Theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(""),
    ];

    for (i, item) in state.menu.items.iter().enumerate() {
        let style = if i == state.menu.selected_index {
            Style::default()
                .fg(Theme::SELECTED)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Theme::TEXT)
        };

        let prefix = if i == state.menu.selected_index {
            "> "
        } else {
            "  "
        };

        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, item),
            style,
        )));
    }

    let menu = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Theme::ACCENT)),
    );

    f.render_widget(menu, menu_area);
}
