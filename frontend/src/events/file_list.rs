use crate::api;
use crate::state::{AppState, Pane};
use ratzilla::event::{KeyCode, KeyEvent};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen_futures::spawn_local;

pub fn handle_keys(state: &mut AppState, state_rc: &Rc<RefCell<AppState>>, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => {
            state.focus = Pane::Menu;
            state.status_message = None;
        }
        KeyCode::Char('j') | KeyCode::Down => {
            state.file_list.next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.file_list.previous();
        }
        KeyCode::Enter => {
            if let Some(filename) = state.file_list.selected().cloned() {
                let state_clone = Rc::clone(state_rc);
                spawn_local(async move {
                    match api::fetch_file_content(&filename).await {
                        Ok(content) => {
                            let mut st = state_clone.borrow_mut();
                            st.editor.load_content(filename.clone(), content);
                            st.dirty = false;
                            st.focus = Pane::Editor;
                            st.set_status(format!("Loaded: {}", filename));
                        }
                        Err(e) => {
                            let mut st = state_clone.borrow_mut();
                            st.set_status(format!("Error loading: {:?}", e));
                        }
                    }
                });
            }
        }
        _ => {}
    }
}
