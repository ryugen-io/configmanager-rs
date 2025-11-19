use super::super::types::ContainerActionResponse;
use axum::{Json, http::StatusCode};
use std::time::Duration;
use tokio::process::Command;

/// Execute a docker action (start/stop/restart) on a container
/// Timeout: 120 seconds for long-running operations
pub(super) async fn execute_container_action(
    container_id: &str,
    action: &str,
) -> Result<Json<ContainerActionResponse>, (StatusCode, String)> {
    let docker_cmd = Command::new("docker").args([action, container_id]).output();

    let output = tokio::time::timeout(Duration::from_secs(120), docker_cmd)
        .await
        .map_err(|_| {
            (
                StatusCode::REQUEST_TIMEOUT,
                format!("docker {} timed out", action),
            )
        })?
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("docker {} failed: {}", action, e),
            )
        })?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("docker {} failed: {}", action, error),
        ));
    }

    let past_tense = match action {
        "start" => "started",
        "stop" => "stopped",
        "restart" => "restarted",
        _ => action,
    };

    Ok(Json(ContainerActionResponse {
        success: true,
        message: format!("container {}", past_tense),
    }))
}
