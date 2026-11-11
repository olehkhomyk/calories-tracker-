pub mod dto;
pub mod entity;
pub mod repository;
pub mod service;
mod handlers;

use crate::features::auth::middleware::auth_guard;
use crate::state::AppState;
use axum::routing::{get, post, put};
use axum::{Router, middleware};

pub fn router(state: AppState) -> Router<AppState> {
    let public = Router::new();

    let protected = Router::new()
        .route("/", get(handlers::get_al_users))
        .route("/{id}", get(handlers::get_user_by_id))
        .route("/", post(handlers::create_user))
        .route("/{id}", put(handlers::update_user))
        .route_layer(middleware::from_fn_with_state(state, auth_guard));

    public.merge(protected)
}
