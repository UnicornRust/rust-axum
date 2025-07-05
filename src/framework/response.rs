use axum::{response::IntoResponse, Json};
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
}

impl <T> ApiResponse<T> {

    // 完整自定义返回
    pub fn new(code: i32, message: String, data: Option<T>) -> Self {
        ApiResponse { code, message, data }
    }

    // 正常返回
    pub fn ok<M: AsRef<str>>(message: M, data: Option<T>) -> Self {
        ApiResponse::new(0, String::from(message.as_ref()), data)
    }

    // 异常返回
    pub fn err<M: AsRef<str>>(message: M) -> Self {
        ApiResponse::new(1, String::from(message.as_ref()), None)
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
