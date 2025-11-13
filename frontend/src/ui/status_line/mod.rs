mod left;
mod right;

use crate::state::AppState;
use ratzilla::ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    // Split status line into left and right sections
    // Right needs more space for build date + tech stack
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    left::render(f, state, chunks[0]);
    right::render(f, chunks[1]);
}
