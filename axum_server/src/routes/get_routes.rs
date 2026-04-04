use crate::handlers;
use crate::services;
use std::sync::Arc;
use axum::routing::post;
use tokio::sync::Mutex;

use axum::{
    routing::get,
    Router,
};


pub fn get_user_route(state: Arc<Mutex<services::user_service::AppState>>) -> Router {
    //
    Router::new()
        .route("/register", post(handlers::get_handlers::register_user))
        .with_state(state.clone())
        .route("/user", post(handlers::get_handlers::post_user))
        .with_state(state.clone())
        .route("/users/{id}", 
        get(handlers::get_handlers::get_user).
                       put(handlers::get_handlers::put_user).
                       delete(handlers::get_handlers::delete_user))
        .route("/api", get(handlers::get_handlers::get_api))
        .with_state(state)
}