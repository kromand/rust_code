use serde::{Deserialize, Serialize};

// example how to change snake case to camel case for json
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: u64,
    pub user_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserTokenResponse {
    pub id: u64,
    pub jwttoken: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub data: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUserDto {
    pub id: u64,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ListPostParams {
    pub page: Option<u32>,
    pub size: Option<u32>,
    pub username: Option<String>,
    pub tag: Option<String>,
}