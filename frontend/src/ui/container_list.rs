use crate::{
    state::{AppState, Pane},
    theme::Theme,
};
use ratzilla::ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let is_focused = state.focus == Pane::ContainerList;

    let items: Vec<ListItem> = state
        .container_list
        .containers
        .iter()
        .map(|container| {
            let status_color = match container.state.as_str() {
                "running" => Theme::GREEN,
                "exited" => Theme::OVERLAY1,
                _ => Theme::YELLOW,
            };

            let short_id = &container.id[..12.min(container.id.len())];
            let line = Line::from(vec![
                ratzilla::ratatui::text::Span::styled(
                    format!("{:<12} ", short_id),
                    Style::default().fg(Theme::YELLOW),
                ),
                ratzilla::ratatui::text::Span::styled(
                    format!("{:<15} ", container.name),
                    Style::default().fg(Theme::TEXT),
                ),
                ratzilla::ratatui::text::Span::styled(
                    format!("[{}] ", container.state),
                    Style::default().fg(status_color),
                ),
                ratzilla::ratatui::text::Span::styled(
                    &container.status,
                    Style::default().fg(Theme::SUBTEXT0),
                ),
            ]);

            ListItem::new(line)
        })
        .collect();

    let border_style = if is_focused {
        Style::default().fg(Theme::ACCENT)
    } else {
        Style::default().fg(Theme::OVERLAY1)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Containers ")
        .border_style(border_style);

    let list = List::new(items).block(block).highlight_style(
        Style::default()
            .bg(Theme::ACCENT)
            .add_modifier(Modifier::BOLD),
    );

    let mut list_state = ListState::default();
    list_state.select(Some(state.container_list.selected_index));

    f.render_stateful_widget(list, area, &mut list_state);
}
