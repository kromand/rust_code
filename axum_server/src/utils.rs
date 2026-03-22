use serde::Serialize;
use axum::response::{IntoResponse,Response};
use axum::http::StatusCode;

#[derive(Debug,Serialize)]
pub struct ApiError {
    pub code:String,
    pub message:String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.code.as_str() {
            "bad_request" => StatusCode::BAD_REQUEST,
            "unauthorized" => StatusCode::UNAUTHORIZED,
            "not_found" => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = serde_json::to_string(&self).unwrap_or_else(|_| "{\"code\":\"internal\",\"message\":\"Server error\"}".into());
        (status,body).into_response()
    }
}