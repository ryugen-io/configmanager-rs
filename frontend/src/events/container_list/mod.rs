mod actions;
mod details;
mod navigation;

use crate::state::{AppState, Pane};
use ratzilla::event::KeyEvent;
use std::{cell::RefCell, rc::Rc};

pub fn handle_keys(state: &mut AppState, state_rc: &Rc<RefCell<AppState>>, key_event: KeyEvent) {
    let keybinds = &state.keybinds.container_list;

    if super::key_matches(&key_event, &keybinds.navigate_down)
        || super::key_matches(&key_event, &keybinds.navigate_down_alt)
    {
        navigation::next(state);
    } else if super::key_matches(&key_event, &keybinds.navigate_up)
        || super::key_matches(&key_event, &keybinds.navigate_up_alt)
    {
        navigation::previous(state);
    } else if super::key_matches(&key_event, &keybinds.start_container) {
        actions::start_container(state, state_rc);
    } else if super::key_matches(&key_event, &keybinds.stop_container) {
        actions::stop_container(state, state_rc);
    } else if super::key_matches(&key_event, &keybinds.restart_container) {
        actions::restart_container(state, state_rc);
    } else if super::key_matches(&key_event, &keybinds.back_to_menu) {
        state.focus = Pane::Menu;
    } else {
        // Enter to view details (not configurable for now)
        if super::match_key_without_mods(&key_event, "Enter") {
            details::load_details(state, state_rc);
        }
    }
}
