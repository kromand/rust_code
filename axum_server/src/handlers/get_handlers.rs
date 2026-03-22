use crate::utils::ApiError;
use axum::{Json, extract::Path};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub username: String,
}

#[axum::debug_handler]
pub async fn get_user(Path(user_id): Path<u64>) -> Result<Json<UserResponse>, ApiError> {
    //let user = user_service.get(user_id).await;
    Ok(Json(UserResponse {
        id: user_id,
        username: "bla".to_string(),
    }))
}
