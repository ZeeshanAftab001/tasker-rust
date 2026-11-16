use crate::{AppState, handlers,logging};
use axum::{
    Router,
    routing::{delete, get, patch, post},
    middleware::from_fn
};
use std::error::Error;

pub async fn create_router() -> Result<Router<AppState>, Box<dyn Error>> {
    Ok(Router::new()
        .route("/{id}", get(handlers::task::get_todo)).layer(from_fn(logging::task::logger))
        .route("/all", get(handlers::task::get_all_todo)).layer(from_fn(logging::task::logger))
        .route("/create", post(handlers::task::add_todo)).layer(from_fn(logging::task::logger))
        .route("/update/{id}", patch(handlers::task::update_todo)).layer(from_fn(logging::task::logger))
        .route("/delete/{id}", delete(handlers::task::delete_todo)).layer(from_fn(logging::task::logger)))
}
