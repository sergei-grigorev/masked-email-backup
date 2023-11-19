use config::{Config, ConfigError};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::{
    fs::File,
    io::{self, BufWriter},
};

use super::{AppConfig, ConfigReader};

pub struct UserConfig();

const CONFIG_PATH: &'static str = "maskedemail-cli.toml";
const USER_NAME_PARAM: &'static str = "user_name";
const STORAGE_PARAM: &'static str = "storage";

impl UserConfig {
    /// Get the full path for the config file.
    fn derive_config_path() -> PathBuf {
        match dirs::config_dir() {
            Some(conf) => {
                let mut target = conf;
                target.push(CONFIG_PATH);
                target
            }
            None => PathBuf::from(CONFIG_PATH),
        }
    }

    fn load(config_path: &Path) -> Result<AppConfig, ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::from(config_path))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        log::debug!("Config file exists, start to parse it");

        let user_name = settings.get_string(USER_NAME_PARAM)?;
        let storage = settings.get_string(STORAGE_PARAM)?;
        Ok(AppConfig { user_name, storage })
    }

    fn update(config: &AppConfig, config_path: &Path) -> Result<(), io::Error> {
        let file = File::create(config_path)?;
        let mut buf_writer = BufWriter::new(file);
        writeln!(buf_writer, "{} = \"{}\"", USER_NAME_PARAM, config.user_name)?;
        writeln!(buf_writer, "{} = \"{}\"", STORAGE_PARAM, config.storage)?;
        buf_writer.flush()?;

        Ok(())
    }
}

impl ConfigReader for UserConfig {
    fn load() -> Result<AppConfig, ConfigError> {
        let config_path: PathBuf = UserConfig::derive_config_path();
        log::info!(
            "Attempt to read the config from [{}]",
            config_path.display()
        );
        UserConfig::load(config_path.as_path())
    }

    fn update(config: &AppConfig) -> Result<(), io::Error> {
        let config_path: PathBuf = UserConfig::derive_config_path();
        log::info!(
            "Configuration will be stored in: [{}]",
            config_path.display()
        );
        UserConfig::update(config, config_path.as_path())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::config::AppConfig;

    use super::UserConfig;

    #[test]
    fn check_the_write_format() {
        let mut tmp_dir = env::temp_dir();
        tmp_dir.push("test_config.toml");

        let sample = AppConfig {
            user_name: "my_user@example.com".to_owned(),
            storage: tmp_dir.as_path().to_str().unwrap().to_owned(),
        };

        UserConfig::update(&sample, &tmp_dir).unwrap();

        let reloaded = UserConfig::load(&tmp_dir).unwrap();

        assert_eq!(reloaded.user_name, sample.user_name);
        assert_eq!(reloaded.storage, sample.storage);
    }
}
