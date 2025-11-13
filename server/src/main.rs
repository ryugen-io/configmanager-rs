mod routes;
mod version;

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    println!("{}", version::version_string());
    let app = Router::new()
        // API routes
        .route("/api/configs", get(routes::list_configs))
        .route("/api/configs/:filename", get(routes::read_config))
        .route("/api/configs/:filename", post(routes::write_config))
        // Static files (frontend)
        .nest_service("/", ServeDir::new("frontend/dist"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://0.0.0.0:3000 (accessible at http://10.1.1.30:3000)");
    println!("API endpoints:");
    println!("  GET  /api/configs");
    println!("  GET  /api/configs/:filename");
    println!("  POST /api/configs/:filename");

    axum::serve(listener, app).await.unwrap();
}
