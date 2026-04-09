use crate::utils::ApiError;
use crate::Config;
use std::collections::HashMap;
use sqlx::postgres::PgPoolOptions;


#[derive(Clone)]
pub struct UserEntry {
    user_id: u64,
    pub user_name: String,
    pub user_email: String,
    pub password: String,
}
impl UserEntry {
    pub fn new(id: u64, name: String, e: String, p:String) -> UserEntry {
        UserEntry {
            user_id: id,
            user_name: name,
            user_email: e,
            password: p,
        }
    }
}
#[derive(Clone)]
pub struct DatabaseSim {
    userdata: HashMap<u64, UserEntry>,
    user_id: u64,
}

impl DatabaseSim {
    pub fn new() -> DatabaseSim {
        DatabaseSim {
            userdata: HashMap::<u64, UserEntry>::new(),
            user_id: 2,
        }
    }
    pub fn add_user(self: &mut DatabaseSim, name: String, e: String, p:String) -> Option<u64> {
        self.user_id += 1;
        let id = self.user_id;

        if self.userdata.contains_key(&id) {
            return None;
        }
        self.userdata.insert(id, UserEntry::new(id, name, e,p));
        Some(id)
    }
    pub fn get_user(self: &DatabaseSim, id: u64) -> Option<UserEntry> {
        if let Some(val) = self.userdata.get(&id) {
            return Some((*val).clone());
        }

        None
    }
    pub fn remove_user(self: &mut DatabaseSim, id: u64) -> Option<UserEntry> {
        self.userdata.remove(&id)
    }
    pub fn change_user(self: &mut DatabaseSim, id: u64, name: String, e: String) -> bool {
        if let Some(val) = self.userdata.get_mut(&id) {
            val.user_email = e;
            val.user_name = name;
            return true;
        }
        false
    }
    pub fn get_user_hashed_password(self: &mut DatabaseSim, id: u64) -> Option<String>
    {
        if let Some(val) = self.userdata.get(&id) {

            return Some(val.password.clone());
        }
        None
    }
}

pub async fn get_user_email(user_id: u64) -> Result<String, ApiError> {
    dbg!(user_id);
    if user_id == 1 {
        return Ok("test@test.com".to_string());
    }
    Err(ApiError {
        code: "not_found".into(),
        message: "user if not found".into(),
    })
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseSim,
    pub database_con_pool: sqlx::PgPool,
    pub config: Config,
}
