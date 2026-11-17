pub mod task;
pub mod user;
use axum::{Router, middleware::from_fn};

use crate::{AppState, logging};


pub async fn get_router() -> Router<AppState> {

    let user_router=user::get_user_routes().await;
    let task_router=task::get_task_router().await;
    Router::new()
    .nest("/api/v1/users", user_router)
    .nest("/api/v1/tasks", task_router)
    .layer(from_fn(logging::task::logger))
}