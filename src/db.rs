use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::PgConnection;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub host: String,
    pub dbname: String,
    pub user: String,
    pub password: String,
    pub uri: String,
}

impl Config {
    pub const HOST: &'static str = "localhost";
    pub const DBNAME: &'static str = "integer";
}

impl Default for Config {
    fn default() -> Self {
        let host = env::var("DB_HOST").unwrap_or_else(|_| Config::HOST.to_string());
        let dbname = env::var("DB_NAME").unwrap_or_else(|_| Config::DBNAME.to_string());
        let user = env::var("DB_USER").unwrap();
        let password = env::var("DB_PASSWORD").unwrap_or_else(|_| {
            let password_file = env::var("DB_PASSWORD_FILE").unwrap();
            fs::read_to_string(password_file)
                .unwrap()
                .trim()
                .to_string()
        });
        let uri = format!("postgresql://{}:{}@{}/{}", user, password, host, dbname);
        Self {
            host,
            dbname,
            user,
            password,
            uri,
        }
    }
}

#[database("integer")]
pub struct Conn(PgConnection);
