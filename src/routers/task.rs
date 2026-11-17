use crate::{AppState, handlers};
use axum::{
    Router,
    routing::{delete, get, patch, post},

};

pub async fn get_task_router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(handlers::task::get_todo))
        .route("/all", get(handlers::task::get_all_todo))
        .route("/create", post(handlers::task::add_todo))
        .route("/update/{id}", patch(handlers::task::update_todo))
        .route("/delete/{id}", delete(handlers::task::delete_todo))
}
