use config::ConfigError;
use std::io;

pub mod userconfig;

pub struct AppConfig {
    pub user_name: String,
    pub storage: String,
}

pub trait ConfigReader {
    /// Load app configuration.
    /// That function returns an error in case the file does not exists.
    fn load() -> Result<AppConfig, ConfigError>;

    /// Create or update the configuration.
    fn update(config: &AppConfig) -> Result<(), io::Error>;
}
