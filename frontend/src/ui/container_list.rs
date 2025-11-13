use crate::{
    state::{AppState, Pane},
    theme::container_list::ContainerListTheme,
};
use ratzilla::ratatui::{
    Frame,
    layout::Rect,
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
            let status_color = ContainerListTheme::status_color(&container.state);

            let short_id = &container.id[..12.min(container.id.len())];
            let line = Line::from(vec![
                ratzilla::ratatui::text::Span::styled(
                    format!("{:<12} ", short_id),
                    ContainerListTheme::id_style(),
                ),
                ratzilla::ratatui::text::Span::styled(
                    format!("{:<15} ", container.name),
                    ContainerListTheme::name_style(),
                ),
                ratzilla::ratatui::text::Span::styled(
                    format!("[{}] ", container.state),
                    ratzilla::ratatui::style::Style::default().fg(status_color),
                ),
                ratzilla::ratatui::text::Span::styled(
                    &container.status,
                    ContainerListTheme::status_info_style(),
                ),
            ]);

            ListItem::new(line)
        })
        .collect();

    let border_style = if is_focused {
        ContainerListTheme::border_focused()
    } else {
        ContainerListTheme::border_unfocused()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Containers ")
        .border_style(border_style);

    let list = List::new(items)
        .block(block)
        .highlight_style(ContainerListTheme::highlight_style());

    let mut list_state = ListState::default();
    list_state.select(Some(state.container_list.selected_index));

    f.render_stateful_widget(list, area, &mut list_state);
}
