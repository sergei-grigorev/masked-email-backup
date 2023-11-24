use actions::{print_emails, refresh_db};
use config::{
    run_args, userconfig::UserConfig, AppConfig, ConfigReader, COMMAND_INIT, COMMAND_PRINT_DB,
    COMMAND_REFRESH_DB, COMMAND_UPDATE_PASSWORD,
};
use secrets::fake::FakeSecret;

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

    run_app::<FakeSecret, UserConfig>();
}

fn run_app<PasswordStorage, ConfigStorage>() -> ()
where
    PasswordStorage: SecureStorage,
    ConfigStorage: ConfigReader,
{
    let args = run_args().get_matches();
    match args.subcommand_name() {
        Some(init) if init == COMMAND_INIT => {
            // create a new configuration
            let user_name: String = user_prompt("Please enter your user name").unwrap();
            let directory: String = user_prompt("Please enter your database location").unwrap();

            let new_config = AppConfig {
                user_name: user_name.to_owned(),
                storage: directory.to_owned(),
            };

            ConfigStorage::update(&new_config).expect("Problem with the config update");
        }
        Some(pass) if pass == COMMAND_UPDATE_PASSWORD => {
            // load config
            let config: AppConfig =
                ConfigStorage::load().expect("Configuration is not created or corrupted");

            let password: PasswordValue =
                password_prompt("Please provide your fastmail app specific password").unwrap();
            PasswordStorage::update_password(&config.user_name, &password)
                .expect("Password was not stored");

            log::info!("Token was stored in keychain");
        }
        Some(fetch) if fetch == COMMAND_REFRESH_DB => {
            // load config
            let config: AppConfig =
                ConfigStorage::load().expect("Configuration is not created or corrupted");
            match refresh_db::<PasswordStorage>(&config) {
                Ok(()) => (),
                Err(err) => println!("Operation failed: {err}"),
            }
        }
        Some(fetch) if fetch == COMMAND_PRINT_DB => {
            // load config
            let config: AppConfig =
                ConfigStorage::load().expect("Configuration is not created or corrupted");
            match print_emails::<PasswordStorage>(&config) {
                Ok(()) => (),
                Err(err) => println!("Operation failed: {err}"),
            }
        }
        Some(_) => {
            run_args().render_help();
        }
        None => {}
    }
}
