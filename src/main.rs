use std::path::PathBuf;

use actions::{export_lua, refresh_db, show_emails};
use config::{
    run_args, userconfig::UserConfig, AppConfig, ConfigReader, COMMAND_EXPORT_LUA, COMMAND_INIT,
    COMMAND_REFRESH_DB, COMMAND_SHOW_DB, COMMAND_UPDATE_PASSWORD,
};
use secrets::keychain::KeyChain;

use crate::{
    cli::{password_prompt, user_prompt},
    secrets::{fastmail::SecureStorage, PasswordValue},
};

mod actions;
mod cli;
mod config;
mod db;
mod fastmail;
mod model;
mod secrets;

fn main() {
    env_logger::init();

    run_app::<KeyChain, UserConfig>();
}

fn run_app<PasswordStorage, ConfigStorage>()
where
    PasswordStorage: SecureStorage,
    ConfigStorage: ConfigReader,
{
    // load config
    let config: Result<AppConfig, _> = ConfigStorage::try_load();

    let args = run_args().get_matches();
    match args.subcommand() {
        Some((COMMAND_INIT, _)) => {
            // create a new configuration
            let user_name: String = user_prompt("Please enter your user name").unwrap();
            let directory: String = user_prompt("Please enter your database location").unwrap();

            let new_config = AppConfig {
                user_name: user_name.to_owned(),
                storage: directory.to_owned(),
            };

            ConfigStorage::update(&new_config).expect("Problem with the config update");
        }
        Some((COMMAND_UPDATE_PASSWORD, _)) => {
            let password: PasswordValue =
                password_prompt("Please provide your fastmail app specific password").unwrap();

            let config: AppConfig = config.expect("Configuration is not created or corrupted");
            PasswordStorage::update_password(&config.user_name, &password)
                .expect("Password was not stored");

            log::info!("Token was stored in keychain");
        }
        Some((COMMAND_REFRESH_DB, _)) => {
            let config: AppConfig = config.expect("Configuration is not created or corrupted");
            match refresh_db::<PasswordStorage>(&config) {
                Ok(()) => (),
                Err(err) => eprintln!("Operation failed: {err}"),
            }
        }
        Some((COMMAND_EXPORT_LUA, args)) => {
            let config: AppConfig = config.expect("Configuration is not created or corrupted");
            let lua_script = args
                .get_one::<String>("path")
                .expect("Lua script path is not provided")
                .to_owned();
            let path = PathBuf::from(lua_script);
            match export_lua::<PasswordStorage>(&config, &path.as_path()) {
                Ok(()) => (),
                Err(err) => eprintln!("Operation failed: {err}"),
            }
        }
        Some((COMMAND_SHOW_DB, _)) => {
            let config: AppConfig = config.expect("Configuration is not created or corrupted");
            match show_emails::<PasswordStorage>(&config) {
                Ok(()) => (),
                Err(err) => eprintln!("Operation failed: {err}"),
            }
        }
        Some(_) => {
            run_args().render_help();
        }
        None => {}
    }
}
