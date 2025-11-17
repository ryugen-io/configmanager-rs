mod config;
mod routes;
mod version;

use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    println!("{}", version::version_string());

    // Load environment variables from .sys/env/.env if it exists
    if let Err(e) = dotenvy::from_filename(".sys/env/.env") {
        eprintln!("Warning: Could not load .sys/env/.env: {}", e);
        eprintln!("Using default configuration values");
    }

    // Load configuration
    let app_config = match config::AppConfig::load() {
        Ok(cfg) => Arc::new(cfg),
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            eprintln!("Make sure config-manager.toml exists in the current directory");
            std::process::exit(1);
        }
    };

    println!(
        "Loaded {} config files from config-manager.toml",
        app_config.list_files().len()
    );
    let app = Router::new()
        // API routes
        .route("/api/configs", get(routes::list_configs))
        .route("/api/configs/*filename", get(routes::read_config))
        .route("/api/configs/*filename", post(routes::write_config))
        .route("/api/containers", get(routes::list_containers))
        .route(
            "/api/containers/:id/details",
            get(routes::get_container_details),
        )
        .route("/api/containers/:id/start", post(routes::start_container))
        .route("/api/containers/:id/stop", post(routes::stop_container))
        .route(
            "/api/containers/:id/restart",
            post(routes::restart_container),
        )
        // Pass config as state
        .with_state(app_config)
        // Static files (frontend)
        .nest_service("/", ServeDir::new("frontend/dist"));

    // Read server configuration from environment or use defaults
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_addr = format!("0.0.0.0:{}", server_port);
    let display_addr = format!("http://{}:{}", server_host, server_port);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    println!("Server running on {}", display_addr);
    println!("API endpoints:");
    println!("  GET  /api/configs");
    println!("  GET  /api/configs/*filename");
    println!("  POST /api/configs/*filename");
    println!("  GET  /api/containers");
    println!("  POST /api/containers/:id/start");
    println!("  POST /api/containers/:id/stop");
    println!("  POST /api/containers/:id/restart");

    axum::serve(listener, app).await.unwrap();
}
