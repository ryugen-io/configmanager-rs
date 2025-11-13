mod editor;
mod file_list;
mod menu;
mod status_line;

use crate::state::{AppState, Pane};
use ratzilla::ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

pub fn render(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Status line
        ])
        .split(f.area());

    // Main content depends on current pane
    match state.focus {
        Pane::Menu => menu::render(f, state, chunks[0]),
        _ => render_main_content(f, state, chunks[0]),
    }

    // Status line
    status_line::render(f, state, chunks[1]);
}

fn render_main_content(f: &mut Frame, state: &AppState, area: ratzilla::ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // File list
            Constraint::Percentage(75), // Editor
        ])
        .split(area);

    file_list::render(f, state, chunks[0]);
    editor::render(f, state, chunks[1]);
}
