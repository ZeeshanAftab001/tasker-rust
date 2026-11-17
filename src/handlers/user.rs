use crate::AppState;
use axum::extract::State;

pub async fn get_all_users() -> String  {
    format!("These are all users")
}