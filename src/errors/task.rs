use axum::{http::{StatusCode, header}, response::IntoResponse, Json};
use serde_json::json;

pub struct TaskError{
    pub message : String,
    pub error_code : Option<i32>,
    pub status_code : StatusCode
}

impl IntoResponse for TaskError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        (status_code,[(header::CONTENT_TYPE,"application/json")], Json(json!({ "StatusCode": self.status_code.as_u16(),"ErrorCode": self.error_code,"Message": self.message })) ).into_response()
    }
}

impl From<sqlx::Error> for TaskError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => TaskError {
                message: "Todo item not found".to_string(),
                error_code: Some(404),
                status_code: StatusCode::NOT_FOUND,
            },
            _ => TaskError {
                message: format!("Database error: {}", err),
                error_code: Some(500),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}