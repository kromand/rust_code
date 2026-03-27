mod handlers;
mod utils;
mod config;
mod routes;
mod services;
mod tests;

use std::sync::Arc;

struct AppState {
    state: u32,
}

#[tokio::main]
async fn main() {
    //TODO: add tracing/ tracing subscriber init for logging 
    //for tokio metrics
    //console_subscriber::init();
    let mut state = Arc::new(AppState {
       state:0, 
    });
    // build our application with a single route
    let app = routes::get_routes::get_user_route();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(config::LOOPBACK_IP).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
