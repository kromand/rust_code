use crate::infrastructure::authentication;
use crate::services;
use crate::dto;
use crate::utils::ApiError;
use crate::infrastructure;

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use axum::{Json,extract::{Path, State},response::IntoResponse,};

/*
Register user: hash login credentials
*/
pub async fn register_user(state: State<Arc<Mutex<services::user_service::AppState>>>,
    Json(new_user): Json<dto::user_dto::CreateUserDto>) -> Result<Json<dto::user_dto::UserResponse>, ApiError> 
{
    info!("register_user handler");

    let mut m = state.lock().await;

    let hash = infrastructure::authentication::hash_password(&new_user.password).unwrap();

    if let Some(id) = m.db.add_user(new_user.username.clone(), new_user.email,hash) {
        Ok(Json(dto::user_dto::UserResponse {
            id,
            user_name: new_user.username,
        }))
    } else {
        Err(ApiError {
            code: "not_found".to_owned(),
            message: "user not found in test db".into(),
        })
    }
}
/*
Login: Check credentials and issue jwt token
*/
pub async fn login_user(state: State<Arc<Mutex<services::user_service::AppState>>>,
    Json(new_user): Json<dto::user_dto::LoginUserDto>) -> Result<Json<dto::user_dto::UserTokenResponse>, ApiError> 
{
    info!("login_user handler");
    let mut m = state.lock().await;

    if let Some(val) = m.db.get_user_hashed_password(new_user.id) {
        if infrastructure::authentication::verify_password(&val, &new_user.password) {

            let roles = vec!["Write".to_string(),"Read".to_string()];

            return Ok(Json(dto::user_dto::UserTokenResponse {
                id: new_user.id,
                jwttoken: infrastructure::authentication::generate_jwt(new_user.id, &m.config.jwt_secret,roles),
            }));
        }
        Err(ApiError {
            code: "unauthorized".to_owned(),
            message: "Incorrect password for this user".into(),
        })
    }
    else {
        Err(ApiError {
            code: "not_found".to_owned(),
            message: "user not found in test db".into(),
        })
    }
}

//create new user data entry
#[axum::debug_handler]
pub async fn post_user(
    State(state): State<Arc<Mutex<services::user_service::AppState>>>,
        cl: authentication::Claims,
        Json(new_user): Json<dto::user_dto::CreateUser>,

) -> Result<Json<dto::user_dto::UserResponse>, ApiError> {

    info!("post_user handler");

    let mut m = state.lock().await;

    if let Some(id) = m.db.add_user(new_user.username.clone(), new_user.email.clone(),"psswd".into()) {

        match infrastructure::db::create_user_db(&m.database_con_pool, id as i32, &new_user).await
        {
            Ok(_) => {
                Ok(Json(dto::user_dto::UserResponse {
                    id,
                    user_name: new_user.username,
                }))
            }
            Err(e) => {
                info!("post_user db insert error");
                Err(ApiError {
                    code: "not_found".to_owned(),
                    message: e.to_string(),
                })
            }
        }    
    } else {
        Err(ApiError {
            code: "not_found".to_owned(),
            message: "user already exists".into(),
        })
    }
}

//get user info
pub async fn get_user(
    State(state): State<Arc<Mutex<services::user_service::AppState>>>,
        cl: authentication::Claims,
        Path(user_id): Path<u64>,
        ) -> Result<Json<dto::user_dto::UserResponse>, ApiError> {

    info!("get_user #1 {}", &user_id);

    let m = state.lock().await;

    if let Some(val) = m.db.get_user(user_id) {
        Ok(Json(dto::user_dto::UserResponse {
            id: user_id,
            user_name: val.user_name.clone(),
        }))
    } else {
        Err(ApiError {
            code: "not_found".to_owned(),
            message: "user not found in test db".into(),
        })
    }
}

//change user info
pub async fn put_user(
    State(state): State<Arc<Mutex<services::user_service::AppState>>>,
    cl: authentication::Claims,
    Path(user_id): Path<u64>,
) -> Result<Json<dto::user_dto::UserResponse>, ApiError> {

    info!("put_user #1 {}", &user_id);

    let m = state.lock().await;

    if let Some(val) = m.db.get_user(user_id) {
        Ok(Json(dto::user_dto::UserResponse {
            id: user_id,
            user_name: val.user_name.clone(),
        }))
    } else {
        Err(ApiError {
            code: "not_found".to_owned(),
            message: "user not found in test db".into(),
        })
    }
}

//delete
pub async fn delete_user(
    State(state): State<Arc<Mutex<services::user_service::AppState>>>,
    cl: authentication::Claims,
    Path(user_id): Path<u64>,
) -> Result<Json<dto::user_dto::UserResponse>, ApiError> {

    info!("delete_user #1 {}", &user_id);

    let mut m = state.lock().await;
    if let Some(val) = m.db.remove_user(user_id) {
        Ok(Json(dto::user_dto::UserResponse {
            id: user_id,
            user_name: val.user_name.clone(),
        }))
    } else {
        Err(ApiError {
            code: "not_found".to_owned(),
            message: "user not found in test db".into(),
        })
    }
}

pub async fn get_api() -> Result<Json<dto::user_dto::UserResponse>, ApiError> {
    println!("get_api ");
    Ok(Json(dto::user_dto::UserResponse {
        id: 1,
        user_name: "user".into(),
    }))
}

pub async fn create_user(Json(payload): Json<dto::user_dto::CreateUser>) -> impl IntoResponse {
    println!("create_user ");
}

pub async fn get_user_name(Json(payload): Json<dto::user_dto::CreateUser>) -> impl IntoResponse {
    println!("get_user_name ");
    let user = dto::user_dto::UserResponse {
        id: 1,
        user_name: "name".into(),
    };
    Json(user)
}
