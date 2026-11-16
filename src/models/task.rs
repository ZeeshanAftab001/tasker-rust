use serde::{Deserialize,Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;


#[derive(Debug, sqlx::FromRow,Serialize)]
pub struct Todo{
    pub id : Uuid,
    pub title : String,
    pub content : Option<String>,
    pub status : bool,
    pub created_at : DateTime<Utc>,
    pub updated_at : DateTime<Utc>
}

#[derive(Debug,Deserialize)]
pub struct CreateTodo{
    pub title : String,
    pub content : String,
    pub status : bool
}

#[derive(Debug,Deserialize)]
pub struct UpdateTodo{
    pub title : Option<String>,
    pub content : Option<String>,
    pub status : Option<bool>
}