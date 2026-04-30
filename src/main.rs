// This tells Rust: "Look for a file named handlers.rs"
mod handlers;
mod services;
mod models;
mod blog_logic;

// Use the structs in main to set up the server
use models::AppState;

use axum::{
    http::{header, HeaderValue},
    routing::get,
    Router,
};
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

// --- Main ---

#[tokio::main]
async fn main() {
    // 1. Create Tera
    let tera = Tera::new("templates/*.html").expect("Error parsing templates");

    // Initialize github service
    let github_service = crate::models::GitHubService::new("maurobusso");

    let shared_state = Arc::new(AppState {
        templates: tera,
        github_service,
    });

    // Security Headers Middleware
    let security_layer = SetResponseHeaderLayer::if_not_present(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static("default-src 'self'; script-src 'self' plausible.io; style-src 'self' 'unsafe-inline' fonts.googleapis.com; font-src fonts.gstatic.com; img-src 'self' data:"),
    );

    let app = Router::new()
        .route("/", get(handlers::home_handler))
        .route("/health", get(handlers::health_handler))
        .route("/blog", get(handlers::blog_handler))
        .route("/blog/:slug", get(handlers::blog_post_handler))
        .route("/experience", get(handlers::experience_handler))
        .route("/contact", get(handlers::contact_handler))
        .route("/projects", get(handlers::projects_handler))
        .nest_service("/static", ServeDir::new("static"))
        .layer(security_layer)
        .with_state(shared_state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    println!("Server starting on :{}", port);
    axum::serve(listener, app).await.unwrap();
}