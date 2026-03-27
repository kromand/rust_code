use crate::handlers;
use axum::{
    routing::get,
    Router,
};


pub fn get_user_route() -> Router {
    Router::new()
        .route("/users/user_id/{id}", get(handlers::get_handlers::get_user))
        .route("/api", get(handlers::get_handlers::get_api))
}