mod api;
mod events;
mod state;
mod storage;
mod theme;
mod ui;

use ratzilla::{DomBackend, WebRenderer};
use state::{AppState, Pane};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    // Initialize app state
    let app_state = Rc::new(RefCell::new(AppState::new()));

    // Initialize Ratzilla backend and terminal
    let backend = DomBackend::new().map_err(|e| JsValue::from_str(&e.to_string()))?;
    let terminal =
        ratzilla::ratatui::Terminal::new(backend).map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Show welcome message and load file list if needed
    {
        let state = app_state.borrow();
        let should_load_files = state.focus != Pane::Menu;
        drop(state);

        if should_load_files {
            // Load file list if we restored to FileList or Editor
            let state_clone = Rc::clone(&app_state);
            spawn_local(async move {
                match api::fetch_file_list().await {
                    Ok(files) => {
                        let mut st = state_clone.borrow_mut();
                        st.file_list.set_files(files);
                        st.set_status("Restored session");
                    }
                    Err(e) => {
                        let mut st = state_clone.borrow_mut();
                        st.set_status(format!("Error loading files: {:?}", e));
                    }
                }
            });
        } else {
            let mut state = app_state.borrow_mut();
            state.set_status("Welcome to Config Manager");
        }
    }

    // Set up key event handler
    terminal.on_key_event({
        let state_clone = Rc::clone(&app_state);
        move |key_event| {
            events::handle_key_event(Rc::clone(&state_clone), key_event);
        }
    });

    // Set up drawing loop
    terminal.draw_web(move |f| {
        let state = app_state.borrow();
        ui::render(f, &state);
    });

    Ok(())
}
