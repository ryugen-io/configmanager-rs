use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Deserialize)]
struct FileListResponse {
    files: Vec<String>,
}

#[derive(Deserialize)]
struct FileContentResponse {
    content: String,
}

#[derive(Serialize)]
struct WriteConfigRequest {
    content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub state: String,
    pub status: String,
}

#[derive(Deserialize)]
struct ContainerListResponse {
    containers: Vec<ContainerInfo>,
}

pub async fn fetch_file_list() -> Result<Vec<String>, JsValue> {
    let response = Request::get("/api/configs")
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to fetch file list: {}", e)))?;

    if !response.ok() {
        return Err(JsValue::from_str(&format!(
            "Server returned error: {}",
            response.status()
        )));
    }

    let data: FileListResponse = response
        .json()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

    Ok(data.files)
}

pub async fn fetch_file_content(filename: &str) -> Result<String, JsValue> {
    let url = format!("/api/configs/{}", filename);
    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to fetch file: {}", e)))?;

    if !response.ok() {
        return Err(JsValue::from_str(&format!(
            "Server returned error: {}",
            response.status()
        )));
    }

    let data: FileContentResponse = response
        .json()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

    Ok(data.content)
}

pub async fn save_file_content(filename: &str, content: String) -> Result<(), JsValue> {
    let url = format!("/api/configs/{}", filename);
    let payload = WriteConfigRequest { content };

    let response = Request::post(&url)
        .json(&payload)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize JSON: {}", e)))?
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to save file: {}", e)))?;

    if !response.ok() {
        return Err(JsValue::from_str(&format!(
            "Server returned error: {}",
            response.status()
        )));
    }

    Ok(())
}

pub async fn fetch_container_list() -> Result<Vec<ContainerInfo>, JsValue> {
    let response = Request::get("/api/containers")
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to fetch containers: {}", e)))?;

    if !response.ok() {
        return Err(JsValue::from_str(&format!(
            "Server returned error: {}",
            response.status()
        )));
    }

    let data: ContainerListResponse = response
        .json()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

    Ok(data.containers)
}
