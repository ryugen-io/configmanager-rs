use super::{AppState, Pane};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen_futures::spawn_local;

/// Refresh data for a specific pane
pub fn refresh_pane(pane: Pane, state_rc: &Rc<RefCell<AppState>>) {
    match pane {
        Pane::FileList => refresh_file_list(state_rc),
        Pane::ContainerList => refresh_container_list(state_rc),
        _ => {}
    }
}

/// Load cached data for a pane from storage
pub fn load_pane_cache(pane: Pane, state: &mut AppState) {
    match pane {
        Pane::FileList => {
            if let Some(files) = crate::storage::generic::load("file-list") {
                state.file_list.set_files(files);
            }
        }
        Pane::ContainerList => {
            if let Some(containers) = crate::storage::generic::load("container-list") {
                state.container_list.set_containers(containers);
            }
        }
        _ => {}
    }
}

fn refresh_file_list(state_rc: &Rc<RefCell<AppState>>) {
    let state_clone = Rc::clone(state_rc);
    spawn_local(async move {
        match crate::api::fetch_file_list().await {
            Ok(files) => {
                crate::storage::generic::save("file-list", &files);
                let mut st = state_clone.borrow_mut();
                st.file_list.set_files(files);
                st.set_status("Loaded file list");
            }
            Err(e) => {
                crate::storage::generic::clear("file-list");
                let mut st = state_clone.borrow_mut();
                st.set_status(format!("Error loading files: {:?}", e));
            }
        }
    });
}

fn refresh_container_list(state_rc: &Rc<RefCell<AppState>>) {
    let state_clone = Rc::clone(state_rc);
    spawn_local(async move {
        match crate::api::fetch_container_list().await {
            Ok(containers) => {
                crate::storage::generic::save("container-list", &containers);
                let mut st = state_clone.borrow_mut();
                st.container_list.set_containers(containers);
                st.set_status("Loaded container list");
            }
            Err(e) => {
                crate::storage::generic::clear("container-list");
                let mut st = state_clone.borrow_mut();
                st.set_status(format!("Error loading containers: {:?}", e));
            }
        }
    });
}
