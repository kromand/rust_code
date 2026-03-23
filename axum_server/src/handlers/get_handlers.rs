use crate::utils::ApiError;
use crate::services;

use axum::{Json, extract::Path};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub username: String,
}

#[axum::debug_handler]
pub async fn get_user(Path(user_id): Path<u64>) -> Result<Json<UserResponse>, ApiError> {
    let user = services::user_service::get_user_email(user_id).await?;
    Ok(Json(UserResponse {
        id: user_id,
        username: user,
    }))
}
