use axum::{extract::{State,Json},http::StatusCode};
use crate::{AppState, models::task::Todo};
use uuid::Uuid;

pub async fn get_one(
    State(state): State<AppState>,
    id: Uuid,
) -> Result<Json<Todo>, StatusCode> {
    let todo = sqlx::query_as!(
        Todo,
        "SELECT * FROM Todo WHERE id = $1",
        id
    )
    .fetch_optional(&state.pgpool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
pub async fn create(state : State<AppState>,id:Uuid,title:String,content:String,completed:bool){
    sqlx::query_as::<_,Todo>(
        "INSERT INTO todo(id,title,content,completed) VALUES($1,$2,$3,$4) 
        RETURNING id,title",
    ).bind(id)
    .bind(title)
    .bind(content)
    .bind(completed)
    .fetch_one(&state.pgpool)
    .await.expect("Error create todo");
}
pub async fn get_all(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM Todo;")
        .fetch_all(&state.pgpool)
        .await
        .unwrap_or_default();

    Json(todos)
}
pub async fn update(state : State<AppState>,id:Uuid,title : Option<String>,content : Option<String>,completed:Option<bool>) -> Result<Json<Todo>,StatusCode>{

    let updated_todo=sqlx::query_as::<_,Todo>(
        "UPDATE Todo 
        SET title=COALESCE($1,title),
        content=COALESCE($2,content),
        completed=COALESCE($3,completed)
        WHERE id=$4 RETURNING *",
    ).bind(title)
    .bind(content)
    .bind(completed)
    .bind(id)
    .fetch_optional(&state.pgpool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).expect("Error Updating Todo");
    
    match updated_todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }

}
pub async fn delete(
    State(state): State<AppState>,id:Uuid) -> Result<Json<Todo>, StatusCode> {
    let deleted_todo = sqlx::query_as::<_, Todo>(
        "DELETE FROM Todo WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_optional(&state.pgpool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match deleted_todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }
}