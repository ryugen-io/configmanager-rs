use web_sys::window;

fn get_local_storage() -> Option<web_sys::Storage> {
    window()?.local_storage().ok()?
}

/// Save any serializable data to localStorage
pub fn save<T: serde::Serialize>(key: &str, value: &T) {
    if let Some(storage) = get_local_storage()
        && let Ok(json) = serde_json::to_string(value)
    {
        let _ = storage.set_item(key, &json);
    }
}

/// Load any deserializable data from localStorage
pub fn load<T: serde::de::DeserializeOwned>(key: &str) -> Option<T> {
    let storage = get_local_storage()?;
    let json = storage.get_item(key).ok()??;
    serde_json::from_str(&json).ok()
}

/// Clear data from localStorage
pub fn clear(key: &str) {
    if let Some(storage) = get_local_storage() {
        let _ = storage.remove_item(key);
    }
}
