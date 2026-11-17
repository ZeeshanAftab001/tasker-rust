use axum::{Router,routing::get};
use crate::{AppState, handlers::user};


pub async fn get_user_routes() -> Router<AppState> {

    Router::new().route("/all",get(user::get_all_users))

}