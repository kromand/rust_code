mod handlers;
mod utils;

use std::sync::Arc;

struct AppState {
    state: u32,
}

use axum::{
    body::Body,
    extract::State,
    routing::get,
    response::{IntoResponse,Response,Json},
    Router,
    extract::Path,
    http::StatusCode,
};

#[tokio::main]
async fn main() {
    let mut state = Arc::new(AppState {
       state:0, 
    });
    // build our application with a single route
    let app = Router::new()
    .route("/users/:id", get(handlers::get_handlers::get_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
