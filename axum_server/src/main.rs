use std::sync::Arc;


use axum::{
    body::Body,
    extract::State,
    routing::get,
    response::{IntoResponse,Response,Json},
    Router,
    extract::Path,
    http::StatusCode,
};

use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize)]
pub struct UserResponse {
    pub id:u64,
    pub username:String,
} 
struct AppState {
    state: u32,
}

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
//handlers
#[axum::debug_handler]
async fn get_user(Path(user_id):Path<u64>) ->Result<Json<UserResponse>,ApiError> {
    //let user = user_service.get(user_id).await;
    Ok(Json(
        UserResponse {
            id:user_id,
            username:"bla".to_string()
        }
    ))
}

#[tokio::main]
async fn main() {
    let mut state = Arc::new(AppState {
       state:0, 
    });
    // build our application with a single route
    let app = Router::new()
    .route("/users/:id", get(get_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
