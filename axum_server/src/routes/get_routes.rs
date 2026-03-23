use crate::handlers;
use axum::{
    routing::get,
    Router,
};


pub fn get_user_route() -> Router {
    Router::new().route("/users/:id", get(handlers::get_handlers::get_user))
}