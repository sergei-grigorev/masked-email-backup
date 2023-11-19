use std::io::{stdout, Write};

use config::{userconfig::UserConfig, AppConfig, ConfigReader};
use fastmail::FastMailClient;
use secrets::fake::FakeSecret;

use crate::secrets::{fastmail::SecureStorage, PasswordValue};

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
        let mut user_name = String::new();
        let mut directory = String::new();

        println!(
            "Configuration is not found or corrupted. Please provide the params to make a new one"
        );

        print!("Please enter your user name: ");
        stdout().flush().expect("Problem with the terminal");

        std::io::stdin()
            .read_line(&mut user_name)
            .expect("Problem with reading from stdio");

        print!("Please enter your database location: ");
        stdout().flush().expect("Problem with the terminal");
        std::io::stdin()
            .read_line(&mut directory)
            .expect("Problem with reading from stdio");

        let new_config = AppConfig {
            user_name: user_name.trim_end().to_owned(),
            storage: directory.trim_end().to_owned(),
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
            let fast_mail_password =
                rpassword::prompt_password("Please provide your fastmail app specific password: ")
                    .expect("Problem with reading from stdio");
            let password = PasswordValue {
                value: fast_mail_password,
            };

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
