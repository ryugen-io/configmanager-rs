use crate::state::{AppState, Pane};
use ratzilla::event::{KeyCode, KeyEvent};

pub fn handle_keys(state: &mut AppState, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('j') | KeyCode::Down => {
            state.container_list.next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.container_list.previous();
        }
        KeyCode::Esc => {
            state.focus = Pane::Menu;
        }
        KeyCode::Left if key_event.ctrl => {
            state.focus = Pane::Menu;
        }
        _ => {}
    }
}
