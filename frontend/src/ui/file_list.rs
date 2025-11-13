use crate::{
    state::{AppState, Pane},
    theme::file_list::FileListTheme,
};
use ratzilla::ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let is_focused = state.focus == Pane::FileList;

    let border_style = if is_focused {
        FileListTheme::border_focused()
    } else {
        FileListTheme::border_unfocused()
    };

    let items: Vec<ListItem> = state
        .file_list
        .files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let is_selected = i == state.file_list.selected_index;

            let style = if is_selected {
                FileListTheme::selected_item_style()
            } else {
                FileListTheme::normal_item_style()
            };

            let prefix = if is_selected {
                FileListTheme::selected_prefix()
            } else {
                FileListTheme::normal_prefix()
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
