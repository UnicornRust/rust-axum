use axum::Router;

use crate::{app::AppState, error::{ApiError, ApiResult}};

pub mod user;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest(
            "/api",
            Router::new()
            .nest("/users", user::create_router())
            .fallback(async || -> ApiResult<()> {
                    tracing::warn!("Not Found");
                    Err(ApiError::NotFound)
            })
        ).method_not_allowed_fallback(async || -> ApiResult<()> {
            tracing::warn!("Not Found");
            Err(ApiError::MethodNotAllowed)
        }) 
}
