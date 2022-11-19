use config::{Config, ConfigError, File};
use log::debug;
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub db_uri: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub jwt_ttl: i64,
    pub cors_allowed_origins: Vec<String>,
    pub cors_allowed_headers: Vec<String>,
    pub cors_allowed_methods: Vec<String>,
    pub application_permissions: Vec<String>,
    pub admin_only_permissions: Vec<String>,
}
impl Settings {
    pub fn init_configuration() -> Result<Self, ConfigError> {
        debug!("Initializing settings....");
        let environment = env::var("DEV_BOARD_ENV").unwrap_or_else(|_| "development".into());
        let filename = format!("configuration/{}", environment);
        debug!("loading setting file {}...", &filename);
        let settings = Config::builder()
            // default settings
            .add_source(File::with_name("configuration/default").required(true))
            // here we override previous setting
            .add_source(File::with_name(&filename).required(true))
            .build()?
            .try_deserialize();
        debug!("Settings loaded correctly: {:?}", settings);
        settings
    }
}
