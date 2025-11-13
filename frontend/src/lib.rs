mod api;
mod events;
mod state;
mod theme;
mod ui;
mod version;

use ratzilla::{DomBackend, WebRenderer};
use state::AppState;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

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

    // Show welcome message
    {
        let mut state = app_state.borrow_mut();
        state.set_status("Welcome to Config Manager");
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
