use axum::{extract::{Json, State}, http::{StatusCode}};
use crate::errors::task::TaskError;
use crate::{AppState, models::task::Todo};
use uuid::Uuid;

pub async fn get_one(
    State(state): State<AppState>,
    id: Uuid,
) -> Result<Json<Todo>, TaskError> {
    let todo = sqlx::query_as!(
        Todo,
        "SELECT * FROM Todo WHERE id = $1",
        id
    )
    .fetch_optional(&state.pgpool)
    .await
    .map_err(|err| TaskError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?;


    match todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(TaskError {
            message: format!("Todo with id {} not found", id),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        }),
    }
}

pub async fn create(state : State<AppState>,id:Uuid,title:String,content:String,status:bool) -> Result<Json<Todo>,TaskError> {
    let todo=sqlx::query_as::<_,Todo>(
        "INSERT INTO Todo(id,title,content,status) VALUES($1,$2,$3,$4) 
        RETURNING id,title,content,status,created_at,updated_at",
    ).bind(id)
    .bind(title)
    .bind(content)
    .bind(status)
    .fetch_one(&state.pgpool)
    .await
    .map_err(|err| TaskError{message:err.to_string(),status_code:StatusCode::INTERNAL_SERVER_ERROR,error_code:Some(20)})?;
    
    Ok(Json(todo))
}

pub async fn get_all(State(state): State<AppState>) -> Result<Json<Vec<Todo>>,TaskError> {
    
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM Todo;")
        .fetch_all(&state.pgpool)
        .await
        .map_err(|err| TaskError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?;

    Ok(Json(todos))
}

pub async fn update(state : State<AppState>,id:Uuid,title : Option<String>,content : Option<String>,completed:Option<bool>) -> Result<Json<Todo>, TaskError>{

    let updated_todo=sqlx::query_as::<_,Todo>(
        "UPDATE Todo 
        SET title=COALESCE($1,title),
        content=COALESCE($2,content),
        status=COALESCE($3,status)
        WHERE id=$4 RETURNING *",
    ).bind(title)
    .bind(content)
    .bind(completed)
    .bind(id)
    .fetch_optional(&state.pgpool)
    .await
    .map_err(|err| TaskError {message:err.to_string(),status_code:StatusCode::NOT_FOUND,error_code:Some(20)})?;
    
    match updated_todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(TaskError { message: format!("Todo with {id} Not Found!"), 
            error_code: Some(20), 
            status_code: StatusCode::NOT_FOUND
        })
    }
}

pub async fn delete(
    State(state): State<AppState>,id:Uuid) -> Result<Json<Todo>, TaskError> {
    let deleted_todo = sqlx::query_as::<_, Todo>(
        "DELETE FROM Todo WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_optional(&state.pgpool)
    .await
    .map_err(|err| TaskError{message:err.to_string(),status_code:StatusCode::NOT_FOUND,error_code : Some(20)})?;

    match deleted_todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(TaskError{message:format!("Todo with id {id} not found!"),status_code:StatusCode::NOT_FOUND,error_code : Some(20)}),
    }
}