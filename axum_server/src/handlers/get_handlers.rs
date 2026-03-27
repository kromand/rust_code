use crate::utils::ApiError;
use crate::services;

use axum::{Json, extract::Path, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: u64,
    pub user_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    #[serde(rename="userName")]
    pub username:String,
}

pub async fn get_user(Path(user_id): Path<u64>) -> Result<Json<UserResponse>, ApiError> {
    println!("get_user #1 {}", &user_id);
    let user = services::user_service::get_user_email(user_id).await?;
    Ok(Json(UserResponse {
        id: user_id,
        user_name: user,
    }))
}

pub async fn get_api() -> Result<Json<UserResponse>, ApiError> {
    println!("get_api ");
        Ok(Json(UserResponse {
        id: 1,
        user_name: "user".into(),
    }))
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse
{
    println!("create_user ");
}

pub async fn get_user_name(Json(payload): Json<CreateUser>) -> impl IntoResponse
{
    println!("get_user_name ");
    let user = UserResponse {
        id: 1,
        user_name:"name".into(),
    };
    Json(user)
}