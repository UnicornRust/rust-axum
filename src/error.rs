use axum::Json;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_valid::ValidRejection;
use bcrypt::BcryptError;

use crate::response::ApiResponse;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,

    #[error("Method Not Allowed")]
    MethodNotAllowed,

    #[error("Database Error: {0}")]
    DatabaseErr(#[from] sea_orm::DbErr),

    #[error("查询参数错误: {0}")]
    Query(#[from] QueryRejection),

    #[error("路径参数错误: {0}")]
    Path(#[from] PathRejection),

    #[error("Body参数错误: {0}")]
    Json(#[from] JsonRejection),

    #[error("参数校验错误")]
    Validation(String),

    #[error("密钥 Hash 错误: {0}")]
    Bcrypt(#[from] BcryptError),

    #[error("{0}")]
    Biz(String),

    #[error("Error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl From<ValidRejection<ApiError>> for ApiError {
    fn from(value: ValidRejection<ApiError>) -> Self {
        match value {
            ValidRejection::Valid(errors) => ApiError::Validation(errors.to_string()),
            ValidRejection::Inner(error) => error,
        }
    }
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Query(_)
            | ApiError::Path(_)
            | ApiError::Json(_)
            | ApiError::Validation(_) => StatusCode::BAD_REQUEST,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::DatabaseErr(_) | ApiError::Bcrypt(_) | ApiError::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::Biz(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code();
        let body = Json(ApiResponse::<()>::err(self.to_string()));
        (status_code, body).into_response()
    }
}
