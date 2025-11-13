use axum::{Json, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};
use tokio::process::Command;

// Config file directory - could be made configurable
const CONFIG_DIR: &str = "/tmp/config-manager-configs";

#[derive(Serialize)]
pub struct FileListResponse {
    files: Vec<String>,
}

#[derive(Serialize)]
pub struct FileContentResponse {
    content: String,
}

#[derive(Deserialize)]
pub struct WriteConfigRequest {
    content: String,
}

#[derive(Serialize)]
pub struct WriteConfigResponse {
    success: bool,
}

#[derive(Serialize, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub state: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct ContainerListResponse {
    containers: Vec<ContainerInfo>,
}

// GET /api/configs - List all config files
pub async fn list_configs() -> Result<Json<FileListResponse>, (StatusCode, String)> {
    // Ensure config directory exists
    tokio::fs::create_dir_all(CONFIG_DIR).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create config dir: {}", e),
        )
    })?;

    let mut files = Vec::new();
    let mut dir = tokio::fs::read_dir(CONFIG_DIR).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read directory: {}", e),
        )
    })?;

    while let Some(entry) = dir.next_entry().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read entry: {}", e),
        )
    })? {
        if let Some(filename) = entry.file_name().to_str() {
            // Only include .conf and .toml files
            if filename.ends_with(".conf") || filename.ends_with(".toml") {
                files.push(filename.to_string());
            }
        }
    }

    files.sort();
    Ok(Json(FileListResponse { files }))
}

// GET /api/configs/:filename - Read a config file
pub async fn read_config(
    Path(filename): Path<String>,
) -> Result<Json<FileContentResponse>, (StatusCode, String)> {
    // Security: Validate filename (no path traversal)
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Err((StatusCode::BAD_REQUEST, "Invalid filename".into()));
    }

    // Only allow .conf and .toml files
    if !filename.ends_with(".conf") && !filename.ends_with(".toml") {
        return Err((
            StatusCode::BAD_REQUEST,
            "Only .conf and .toml files allowed".into(),
        ));
    }

    let path = format!("{}/{}", CONFIG_DIR, filename);

    match tokio::fs::read_to_string(&path).await {
        Ok(content) => Ok(Json(FileContentResponse { content })),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err((
            StatusCode::NOT_FOUND,
            format!("File not found: {}", filename),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Read error: {}", e),
        )),
    }
}

// POST /api/configs/:filename - Write a config file
pub async fn write_config(
    Path(filename): Path<String>,
    Json(payload): Json<WriteConfigRequest>,
) -> Result<Json<WriteConfigResponse>, (StatusCode, String)> {
    // Security: Validate filename
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Err((StatusCode::BAD_REQUEST, "Invalid filename".into()));
    }

    // Only allow .conf and .toml files
    if !filename.ends_with(".conf") && !filename.ends_with(".toml") {
        return Err((
            StatusCode::BAD_REQUEST,
            "Only .conf and .toml files allowed".into(),
        ));
    }

    let path = format!("{}/{}", CONFIG_DIR, filename);

    // Create backup before writing (if file exists)
    let backup_path = format!("{}.backup", path);
    let _ = tokio::fs::copy(&path, &backup_path).await;

    match tokio::fs::write(&path, payload.content.as_bytes()).await {
        Ok(_) => Ok(Json(WriteConfigResponse { success: true })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Write error: {}", e),
        )),
    }
}

// GET /api/containers - List all Docker containers
pub async fn list_containers() -> Result<Json<ContainerListResponse>, (StatusCode, String)> {
    // Execute docker ps command
    let output = Command::new("docker")
        .args([
            "ps",
            "-a",
            "--format",
            "{{.ID}}\t{{.Names}}\t{{.State}}\t{{.Status}}",
        ])
        .output()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to execute docker command: {}", e),
            )
        })?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Docker command failed: {}", error),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut containers = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 {
            containers.push(ContainerInfo {
                id: parts[0].to_string(),
                name: parts[1].to_string(),
                state: parts[2].to_string(),
                status: parts[3].to_string(),
            });
        }
    }

    Ok(Json(ContainerListResponse { containers }))
}
