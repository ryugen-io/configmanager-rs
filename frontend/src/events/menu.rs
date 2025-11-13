use crate::api;
use crate::state::{AppState, Pane};
use ratzilla::event::{KeyCode, KeyEvent};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen_futures::spawn_local;

pub fn handle_keys(state: &mut AppState, state_rc: &Rc<RefCell<AppState>>, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('j') | KeyCode::Down => {
            state.menu.next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.menu.previous();
        }
        KeyCode::Enter => {
            if let Some(selected) = state.menu.selected() {
                match selected.as_str() {
                    "Config Files" => {
                        state.focus = Pane::FileList;
                        // Fetch file list if empty
                        if state.file_list.files.is_empty() {
                            let state_clone = Rc::clone(state_rc);
                            spawn_local(async move {
                                match api::fetch_file_list().await {
                                    Ok(files) => {
                                        let mut st = state_clone.borrow_mut();
                                        st.file_list.set_files(files);
                                        st.set_status("Loaded file list");
                                    }
                                    Err(e) => {
                                        let mut st = state_clone.borrow_mut();
                                        st.set_status(format!("Error loading files: {:?}", e));
                                    }
                                }
                            });
                        }
                    }
                    "Container" => {
                        state.focus = Pane::ContainerList;
                        // Fetch container list if empty
                        if state.container_list.containers.is_empty() {
                            let state_clone = Rc::clone(state_rc);
                            spawn_local(async move {
                                match api::fetch_container_list().await {
                                    Ok(containers) => {
                                        let mut st = state_clone.borrow_mut();
                                        st.container_list.set_containers(containers);
                                        st.set_status("Loaded container list");
                                    }
                                    Err(e) => {
                                        let mut st = state_clone.borrow_mut();
                                        st.set_status(format!("Error loading containers: {:?}", e));
                                    }
                                }
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
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
                st.set_status(format!("Error saving: {:?}", e));
            }
        }
    });
}
