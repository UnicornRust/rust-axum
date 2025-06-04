
use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::response::ApiResponse;

pub type ApiResult<T> = Result<T, ApiError>;


#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("Method Not Allowed")]
    MethodNotAllowed,
    #[error("{0}")]
    Biz(String),
    #[error("Error: {0}")]
    Internal(#[from] anyhow::Error)
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self{
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz(_) => StatusCode::BAD_REQUEST,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let  status_code  = self.status_code();
        let body = Json(ApiResponse::<()>::err(self.to_string()));
        (status_code, body).into_response()
    }
}
