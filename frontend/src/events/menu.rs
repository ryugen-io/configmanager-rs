use crate::{
    api,
    state::{AppState, Pane, refresh},
    utils,
};
use ratzilla::event::KeyEvent;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen_futures::spawn_local;

pub fn handle_keys(state: &mut AppState, state_rc: &Rc<RefCell<AppState>>, key_event: KeyEvent) {
    let keybinds = &state.keybinds.menu;

    if super::key_matches(&key_event, &keybinds.navigate_down)
        || super::key_matches(&key_event, &keybinds.navigate_down_alt)
    {
        state.menu.next();
    } else if super::key_matches(&key_event, &keybinds.navigate_up)
        || super::key_matches(&key_event, &keybinds.navigate_up_alt)
    {
        state.menu.previous();
    } else if super::key_matches(&key_event, &keybinds.select)
        && let Some(selected) = state.menu.selected()
    {
        match selected.as_str() {
            "Config Files" => {
                state.focus = Pane::FileList;
                // Always refresh to get latest files from server
                refresh::refresh_pane(Pane::FileList, state_rc);
            }
            "Container" => {
                state.focus = Pane::ContainerList;
                refresh::refresh_pane(Pane::ContainerList, state_rc);
            }
            _ => {}
        }
    }
}

pub fn save_file(state: Rc<RefCell<AppState>>, filename: String, content: String) {
    spawn_local(async move {
        match api::save_file_content(&filename, content.clone()).await {
            Ok(_) => {
                let mut st = state.borrow_mut();
                st.editor.original_content = content;
                st.dirty = false;
                st.set_status(format!("Saved: {}", filename));
            }
            Err(e) => {
                let mut st = state.borrow_mut();
                st.set_status(format!(
                    "[ERROR saving: {}]",
                    utils::error::format_error(&e)
                ));
            }
        }
    });
}
