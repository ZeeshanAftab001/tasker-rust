use axum::extract::{Path,Json,State};
use uuid::Uuid;
use crate::{AppState, errors::task::TaskError, models::task::{CreateTodo, 
                    Todo, UpdateTodo}, services};

pub async fn get_todo(state: State<AppState>,Path(id): Path<Uuid>) -> Result<Json<Todo>, TaskError> {
    services::task::get_one(state, id).await
}

pub async fn add_todo(state : State<AppState>,Json(payload): Json<CreateTodo>) -> Result<Json<Todo>,TaskError> {
    let id=Uuid::new_v4();
    services::task::create(state,id,payload.title, payload.content, payload.status).await
}

pub async fn get_all_todo(state: State<AppState>) -> Result<Json<Vec<Todo>>,TaskError> {
    services::task::get_all(state).await
}

pub async fn update_todo(state : State<AppState>,Path(id): Path<Uuid>,Json(payload): Json<UpdateTodo>) -> Result<Json<Todo>,TaskError> {
    services::task::update(state, id, payload.title, payload.content,payload.status).await
}

pub async fn delete_todo(state : State<AppState>,Path(id): Path<Uuid>) -> Result<Json<Todo>,TaskError>{
    services::task::delete(state,id).await
}