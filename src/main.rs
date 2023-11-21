use config::{
    run_args, userconfig::UserConfig, AppConfig, ConfigReader, COMMAND_INIT, COMMAND_REFRESH_DB,
    COMMAND_UPDATE_PASSWORD,
};
use fastmail::FastMailClient;
use secrets::{fake::FakeSecret, fastmail::FastMailAccount};

use crate::{
    cli::{password_prompt, user_prompt},
    db::{fake::FakeDB, Database},
    secrets::{fastmail::SecureStorage, PasswordValue},
};

mod cli;
mod config;
mod db;
mod fastmail;
mod model;
mod secrets;

fn main() {
    env_logger::init();

    run_app::<FakeSecret, UserConfig, FakeDB>();
}

fn run_app<PasswordStorage, ConfigStorage, DatabaseStorage>() -> ()
where
    PasswordStorage: SecureStorage,
    ConfigStorage: ConfigReader,
    DatabaseStorage: Database,
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
            PasswordStorage::update(&config.user_name, &password).expect("Password was not stored");

            log::info!("Token was stored in keychain");
        }
        Some(fetch) if fetch == COMMAND_REFRESH_DB => {
            // load config
            let config: AppConfig =
                ConfigStorage::load().expect("Configuration is not created or corrupted");
            // load token
            let account: Option<FastMailAccount> = PasswordStorage::load(&config.user_name)
                .expect("Keychain access failed, please add permissions or recreate the password");
            // no problems with keychain but the password is not set up
            let account: FastMailAccount = account.expect("Fastmail password was not setup, please run `masked-email-cli update_password` to create a new one");

            let db = DatabaseStorage::init(config.storage).expect("Database cannot be open");

            // load all masked emails
            match FastMailClient::new(PasswordValue {
                value: account.bearer.value.clone(),
            })
            .and_then(|client| client.load_emails())
            {
                Ok(emails) => {
                    db.store(emails, &config.user_name, &account.into())
                        .expect("Problem storing emails in the database");
                }
                Err(error) => log::error!("Fast Mail connection failed: {:?}", error),
            }
        }
        Some(_) => {
            run_args().render_help();
        }
        None => {}
    }
}
