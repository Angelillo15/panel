use config::{Config, ConfigError, Environment};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub server: Server,
    pub database: DatabaseSettings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseSettings {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn new(path: String) -> Result<Self, ConfigError> {
        let config_path = PathBuf::from(path);

        Config::builder()
            .set_default("server.host", "localhost")?
            .set_default("server.port", 5050)?
            .set_default("database.url", "postgres://postgres:postgres@localhost/postgres")?
            .set_default("database.pool_size", 10)?
            .add_source(config::File::from(config_path))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?
            .try_deserialize()
    }
}
