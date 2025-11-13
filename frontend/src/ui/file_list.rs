use crate::{
    state::{AppState, Pane},
    theme::Theme,
};
use ratzilla::ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let is_focused = state.focus == Pane::FileList;

    let border_style = if is_focused {
        Style::default().fg(Theme::ACCENT)
    } else {
        Style::default().fg(Theme::OVERLAY1)
    };

    let items: Vec<ListItem> = state
        .file_list
        .files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let style = if i == state.file_list.selected_index {
                Style::default()
                    .fg(Theme::SELECTED)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Theme::TEXT)
            };

            let prefix = if i == state.file_list.selected_index {
                "> "
            } else {
                "  "
            };

            ListItem::new(Line::from(vec![Span::styled(
                format!("{}{}", prefix, file),
                style,
            )]))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title("Config Files")
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    f.render_widget(list, area);
}
