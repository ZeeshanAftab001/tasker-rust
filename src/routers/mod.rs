pub mod task;
pub mod user;
use axum::{Router};
use crate::{AppState};
use tower_http::trace::TraceLayer;


pub async fn get_router() -> Router<AppState> {

    let user_router=user::get_user_routes().await;
    let task_router=task::get_task_router().await;
    Router::new()
    .nest("/api/v1/users", user_router)
    .nest("/api/v1/tasks", task_router)
    .layer(TraceLayer::new_for_http())
}