use crate::state::AppState;
use axum::Router;
use axum::routing::post;

pub mod dto;
pub mod service;
pub mod jwt;
pub mod middleware;
mod config;
mod handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(handlers::login))
        .route("/register", post(handlers::register))
}
