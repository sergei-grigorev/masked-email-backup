use config::{
    run_args, userconfig::UserConfig, AppConfig, ConfigReader, COMMAND_INIT,
    COMMAND_UPDATE_PASSWORD,
};
use fastmail::FastMailClient;
use secrets::{fake::FakeSecret, fastmail::FastMailAccount};

use crate::{
    cli::{password_prompt, user_prompt},
    secrets::{fastmail::SecureStorage, PasswordValue},
};

mod cli;
mod config;
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
            println!(
            "Configuration is not found or corrupted. Please provide the params to make a new one"
        );

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
            let config: AppConfig = ConfigStorage::load().expect("Configuration is not created or corrupted. Run `masked-email-cli init` to create a new one");

            let password: PasswordValue =
                password_prompt("Please provide your fastmail app specific password").unwrap();
            PasswordStorage::update(&config.user_name, &password).expect("Password was not stored");

            log::info!("Token was stored in keychain");
        }
        Some(_) => {
            run_args().render_help();
            return;
        }
        None => {
            // load config
            let config: AppConfig = ConfigStorage::load().expect("Configuration is not created or corrupted. Run `masked-email-cli init` to create a new one");
            // load token
            let account: Option<FastMailAccount> = PasswordStorage::load(&config.user_name)
                .expect("Keychain access failed, please add permissions or recreate the password");

            let account: FastMailAccount = account.expect("Fastmail password was not setup, please run `masked-email-cli update_password` to create a new one");

            // load all masked emails
            match FastMailClient::new(account.into()) {
                Ok(client) => {
                    log::info!("Session started");
                    let emails = client.load_emails().unwrap();
                    emails
                        .into_iter()
                        .for_each(|e| println!("{}", serde_json::to_string(&e).unwrap()));
                }
                Err(error) => log::error!("Fast Mail connection failed: {:?}", error),
            }
        }
    }
}
