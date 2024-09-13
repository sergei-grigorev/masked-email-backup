use clap::{Arg, Command};
use config::ConfigError;
use std::io;

pub mod userconfig;

pub const COMMAND_INIT: &str = "init";
pub const COMMAND_UPDATE_PASSWORD: &str = "update-password";
pub const COMMAND_REFRESH_DB: &str = "refresh-db";
pub const COMMAND_EXPORT_LUA: &str = "export-lua";
pub const COMMAND_SHOW_DB: &str = "show";

pub fn run_args() -> Command {
    let mut command = Command::new("masked-email-cli")
        .author("Sergei Grigorev")
        .about("App to see all masked emails created by FastMail service")
        .subcommand(Command::new(COMMAND_INIT).about("Create or update the program configuration"))
        .subcommand(
            Command::new(COMMAND_UPDATE_PASSWORD)
                .about("Store new fastmail password. The old record might be deleted"),
        )
        .subcommand(
            Command::new(COMMAND_REFRESH_DB)
                .about("Download the whole emails list and update the database"),
        )
        .subcommand(
            Command::new(COMMAND_EXPORT_LUA)
                .about("Export all email aliases using provided lua script")
                .arg(Arg::new("path").short('p').required(true)),
        )
        .subcommand(Command::new(COMMAND_SHOW_DB).about("Show all email aliases"));

    command.build();
    command
}

pub struct AppConfig {
    pub user_name: String,
    pub storage: String,
}

pub trait ConfigReader {
    /// Load app configuration.
    /// That function returns an error in case the file does not exists.
    fn try_load() -> Result<AppConfig, ConfigError>;

    /// Create or update the configuration.
    fn update(config: &AppConfig) -> Result<(), io::Error>;
}
