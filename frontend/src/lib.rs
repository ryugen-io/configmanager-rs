mod api;
mod events;
mod keybinds;
mod state;
mod storage;
mod theme;
mod ui;
mod utils;

use ratzilla::{DomBackend, WebRenderer};
use state::{AppState, Pane};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Document, window};

/// Inject font configuration into the document
pub fn inject_font(doc: &Document, font_config: &theme::FontConfig) -> Result<(), JsValue> {
    let head = doc
        .head()
        .ok_or_else(|| JsValue::from_str("No head element found"))?;

    // Inject CDN link if provided
    if let Some(cdn_url) = &font_config.cdn_url {
        // Check if font link already exists (avoid duplicates)
        if doc.query_selector("#theme-font-link")?.is_none() {
            let link = doc.create_element("link")?;
            link.set_attribute("id", "theme-font-link")?;
            link.set_attribute("rel", "stylesheet")?;
            link.set_attribute("href", cdn_url)?;
            head.append_child(&link)?;
        }
    }

    // Inject or update font style
    let style_id = "theme-font-style";
    let font_style = format!(
        "pre {{ font-family: '{}', {}; font-size: {}px; font-weight: {}; margin: 0; }}",
        font_config.family, font_config.fallback, font_config.size, font_config.weight
    );

    if let Some(existing_style) = doc.get_element_by_id(style_id) {
        // Update existing style
        existing_style.set_inner_html(&font_style);
    } else {
        // Create new style element
        let style = doc.create_element("style")?;
        style.set_id(style_id);
        style.set_inner_html(&font_style);
        head.append_child(&style)?;
    }

    Ok(())
}

/// Update DOM elements (background + font) for theme changes
pub fn update_dom_for_theme(theme: &theme::ThemeConfig) -> Result<(), JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("No window"))?;
    let doc = win
        .document()
        .ok_or_else(|| JsValue::from_str("No document"))?;
    let body = doc.body().ok_or_else(|| JsValue::from_str("No body"))?;

    // Set background color
    let mantle = theme.mantle();
    if let ratzilla::ratatui::style::Color::Rgb(r, g, b) = mantle {
        let bg_color = format!("rgb({}, {}, {})", r, g, b);
        body.set_attribute("style", &format!("background-color: {}", bg_color))?;
    }

    // Inject font configuration
    inject_font(&doc, &theme.font)?;

    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    // Initialize app state
    let app_state = Rc::new(RefCell::new(AppState::new()));

    // Set body background color and inject font from theme
    if let Some(win) = window()
        && let Some(doc) = win.document()
        && let Some(body) = doc.body()
    {
        let theme = &app_state.borrow().current_theme;

        // Set background color
        let mantle = theme.mantle();
        if let ratzilla::ratatui::style::Color::Rgb(r, g, b) = mantle {
            let bg_color = format!("rgb({}, {}, {})", r, g, b);
            let _ = body.set_attribute("style", &format!("background-color: {}", bg_color));
        }

        // Inject font configuration
        let font_config = &theme.font;
        if let Err(e) = inject_font(&doc, font_config) {
            web_sys::console::error_1(&JsValue::from_str(&format!(
                "Failed to inject font: {:?}",
                e
            )));
        }
    }

    // Load cached lists from storage
    state::refresh::load_pane_cache(Pane::FileList, &mut app_state.borrow_mut());
    state::refresh::load_pane_cache(Pane::ContainerList, &mut app_state.borrow_mut());

    // Initialize Ratzilla backend and terminal
    let backend = DomBackend::new().map_err(|e| JsValue::from_str(&e.to_string()))?;
    let terminal =
        ratzilla::ratatui::Terminal::new(backend).map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Show welcome message and load data based on restored pane
    {
        let state = app_state.borrow();
        let current_pane = state.focus;
        drop(state);

        match current_pane {
            Pane::FileList | Pane::Editor => {
                // Load file list if we restored to FileList or Editor
                let state_clone = Rc::clone(&app_state);
                spawn_local(async move {
                    match api::fetch_file_list().await {
                        Ok(files) => {
                            let mut st = state_clone.borrow_mut();
                            // Only save to cache if data changed
                            if st.file_list.files != files {
                                storage::generic::save("file-list", &files);
                            }
                            st.file_list.set_files(files);
                            st.set_status("Restored session");
                        }
                        Err(e) => {
                            storage::generic::clear("file-list");
                            state::status_helper::set_status_timed(
                                &state_clone,
                                format!(
                                    "[ERROR loading files: {}]",
                                    utils::error::format_error(&e)
                                ),
                            );
                        }
                    }
                });
            }
            Pane::ContainerList => {
                // Load container list if we restored to ContainerList
                state::refresh::refresh_pane(Pane::ContainerList, &app_state);
                let mut state = app_state.borrow_mut();
                state.set_status("Restored session");
            }
            Pane::Menu => {
                let mut state = app_state.borrow_mut();
                state.set_status("Welcome to Config Manager");
            }
        }
    }

    // Start background refresh for container list (every 10 seconds)
    state::refresh::start_background_refresh(&app_state);

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
