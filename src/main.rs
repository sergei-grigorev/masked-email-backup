use config::{userconfig::UserConfig, AppConfig, ConfigReader};
use fastmail::FastMailClient;
use secrets::fake::FakeSecret;

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
    // load config
    let config: AppConfig;
    if let Ok(parsed_config) = ConfigStorage::load() {
        config = parsed_config;
    } else {
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
        config = new_config;
    }

    // load token
    let pass: PasswordValue;
    match PasswordStorage::load(&config.user_name) {
        Ok(Some(acc)) => {
            log::info!("Successfully loaded token");
            pass = acc.into();
        }
        Ok(None) => {
            let password: PasswordValue =
                password_prompt("Please provide your fastmail app specific password").unwrap();
            PasswordStorage::update(&config.user_name, &password).expect("Password was not stored");

            log::info!("Token was stored in keychain");
            pass = password;
        }
        Err(e) => {
            log::error!("Keychain access failed: {}", e);
            panic!("Cannot continue without token");
        }
    }

    // load all masked emails
    match FastMailClient::new(pass) {
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
