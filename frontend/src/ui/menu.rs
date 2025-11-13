use crate::{state::AppState, theme::menu::MenuTheme};
use ratzilla::ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
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
        Line::from(Span::styled("CONFIG MANAGER", MenuTheme::title_style())),
        Line::from(""),
        Line::from(""),
    ];

    for (i, item) in state.menu.items.iter().enumerate() {
        let is_selected = i == state.menu.selected_index;

        let style = if is_selected {
            MenuTheme::selected_item_style()
        } else {
            MenuTheme::normal_item_style()
        };

        let prefix = if is_selected {
            MenuTheme::selected_prefix()
        } else {
            MenuTheme::normal_prefix()
        };

        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, item),
            style,
        )));
    }

    let menu = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(MenuTheme::border_style()),
    );

    f.render_widget(menu, menu_area);
}
