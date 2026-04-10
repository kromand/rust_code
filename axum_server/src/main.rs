mod config;
mod handlers;
mod routes;
mod services;
mod tests;
mod utils;
mod dto;
mod infrastructure;


use std::sync::Arc;
use tokio::sync::Mutex;
use std::fs::OpenOptions;

use tracing::{info,warn, error};
use tracing_subscriber::{fmt,EnvFilter};

use crate::infrastructure::config::Config;

/*
TODO:
1. https and redirects
2. Paging
*/

pub fn init_logging() {
    let debug_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("axum.log")
        .unwrap();

    tracing_subscriber::fmt()
        .json()
        .with_writer(debug_file)
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .init();
}

#[tokio::main]
async fn main() {
    init_logging();
    
    let config = Config::from_env();

    info!("****************************************** APPLICATION START *********************************************");

    //for tokio metrics
    //console_subscriber::init();
    let state;
    
    match infrastructure::db::init_db_pool(&config.database_url).await {
        Ok(db_pool) => state = Arc::new(Mutex::new(services::user_service::AppState {
            db: services::user_service::DatabaseSim::new(),
            database_con_pool: db_pool,
            config,
        })),
        Err(e) => {
            error!("Failed to initialize db connection pool: {}",e);
            return;
        },
    }
    info!("Postgres connection(s) initialized, staring rest server");
    // build our application with a single route
    let app = routes::get_routes::get_user_route(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(config::LOOPBACK_IP)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
