use crate::api;
use crate::state::{AppState, Pane, refresh, status_helper};
use crate::utils;
use ratzilla::event::KeyEvent;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen_futures::spawn_local;

pub fn handle_keys(state: &mut AppState, state_rc: &Rc<RefCell<AppState>>, key_event: KeyEvent) {
    let keybinds = &state.keybinds.file_list;

    if super::key_matches(&key_event, &keybinds.back_to_menu) {
        state.focus = Pane::Menu;
        state.status_message = None;
    } else if super::key_matches(&key_event, &keybinds.navigate_down)
        || super::key_matches(&key_event, &keybinds.navigate_down_alt)
    {
        state.file_list.next();
        refresh::save_selection(Pane::FileList, state);
    } else if super::key_matches(&key_event, &keybinds.navigate_up)
        || super::key_matches(&key_event, &keybinds.navigate_up_alt)
    {
        state.file_list.previous();
        refresh::save_selection(Pane::FileList, state);
    } else if super::key_matches(&key_event, &keybinds.select)
        && let Some(fileinfo) = state.file_list.selected().cloned()
    {
        let state_clone = Rc::clone(state_rc);
        spawn_local(async move {
            match api::fetch_file_content(&fileinfo.name).await {
                Ok(content) => {
                    let mut st = state_clone.borrow_mut();
                    st.editor.load_content(fileinfo.name.clone(), content);
                    st.dirty = false;
                    st.focus = Pane::Editor;
                    st.set_status("[OK]".to_string());
                }
                Err(e) => {
                    {
                        let mut st = state_clone.borrow_mut();
                        st.editor.clear();
                        st.dirty = false;
                    }
                    status_helper::set_status_timed(
                        &state_clone,
                        format!("[ERROR loading: {}]", utils::error::format_error(&e)),
                    );
                }
            }
        });
    }
}
