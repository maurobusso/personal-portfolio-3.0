use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::sync::Arc;
use tera::Context;

use crate::models::AppState;
use crate::blog_logic::load_blog_posts;

// --- Handlers ---

pub async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn home_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("Title", "Mauro's Portfolio");
    ctx.insert("active_page", "home"); // Added this
    Html(state.templates.render("home.html", &ctx).unwrap())
}

pub async fn blog_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let posts = load_blog_posts();
    let mut ctx = Context::new();
    ctx.insert("Title", "Blog - Mauro's Portfolio");
    ctx.insert("Posts", &posts);
    ctx.insert("active_page", "blog"); // Added this
    Html(state.templates.render("blog.html", &ctx).unwrap())
}

pub async fn blog_post_handler(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let posts = load_blog_posts();
    let post = posts.into_iter().find(|p| p.slug == slug);

    match post {
        Some(p) => {
            let mut ctx = Context::new();
            ctx.insert("Title", &format!("{} - Mauro's Portfolio", p.title));
            ctx.insert("Post", &p);
            ctx.insert("active_page", "blog post");
            Html(state.templates.render("blog-post.html", &ctx).unwrap()).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn contact_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("Title", "Contact - Mauro's Portfolio");
    ctx.insert("active_page", "contact");
    Html(state.templates.render("contact.html", &ctx).unwrap())
}

pub async fn experience_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("Title", "Experience - Mauro's Portfolio");
    ctx.insert("active_page", "experience");
    Html(state.templates.render("experience.html", &ctx).unwrap())
}

pub async fn projects_handler(
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Call your service
    let repos = state.github_service.get_portfolio_repos().await
        .unwrap_or_else(|e| {
            eprintln!("GitHub API error: {}", e);
            vec![] // Return empty list on failure so the page still loads
        });

    let mut ctx = Context::new();
    ctx.insert("Title", "Projects - Mauro's Portfolio");
    ctx.insert("projects", &repos); // Inject the data here!
    ctx.insert("active_page", "projects");

    match state.templates.render("projects.html", &ctx) {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}