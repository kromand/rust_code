use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret:String,
}

impl Config {
    pub fn from_env() ->Self {
        dotenv().ok();
        Config { 
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL missing"), 
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET missing") }
    }
}