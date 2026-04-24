mod config;
mod handlers;
mod routes;
mod services;
mod tests;
mod utils;
mod dto;
mod infrastructure;

use std::sync::Arc;
use std::{fs::{OpenOptions}};
use std::net::SocketAddr;

use anyhow::Ok;
use tokio::sync::Mutex;

use tracing::{info,warn, error};
use tracing_subscriber::{fmt,EnvFilter};

use crate::infrastructure::config::Config;
use axum_server::tls_rustls::RustlsConfig;
/*
all code is heavily based on examples from book Rust rest API deveopment
*/

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
async fn main() -> anyhow::Result<()>{

    let tls_config = RustlsConfig::from_pem_file(
            "./cert.pem",
            "./key.pem",
        )
        .await?;

    init_logging();
    
    let config = Config::from_env();

    info!("****************************************** APPLICATION START *********************************************");

    //for tokio metrics
    //console_subscriber::init();
    let state;
    
    match infrastructure::db::init_db_pool(&config.database_url).await {
        std::result::Result::Ok(db_pool) => {
            state = Arc::new(Mutex::new(services::user_service::AppState {
                db: services::user_service::DatabaseSim::new(),
                database_con_pool: db_pool,
                config,}));
        },
        Err(e) => {
            error!("Failed to initialize db connection pool: {}",e);
            return Ok(());
        },
    };
    info!("Postgres connection(s) initialized, staring rest server");
    // build our application with a single route
    let app = routes::get_routes::get_user_route(state);

    let addr = SocketAddr::from(([192, 168, 1, 2], 443));

    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
