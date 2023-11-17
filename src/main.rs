use config::{fake::FakeConfig, ConfigReader};
use fastmail::FastMailClient;
use secrets::fake::FakeSecret;

use crate::secrets::{fastmail::SecureStorage, PasswordValue};

mod config;
mod fastmail;
mod secrets;

fn main() {
    env_logger::init();

    run_app::<FakeSecret, FakeConfig>();
}

fn run_app<PasswordStorage, ConfigStorage>() -> ()
where
    PasswordStorage: SecureStorage,
    ConfigStorage: ConfigReader,
{
    // load config
    let config = ConfigStorage::load().expect("Configuration loading failed");

    // load token
    let pass: PasswordValue;
    match PasswordStorage::load(&config.user_name) {
        Ok(Some(acc)) => {
            log::info!("Successfully loaded token");
            pass = acc.into();
        }
        Ok(None) => {
            let mut password = PasswordValue {
                value: String::new(),
            };

            println!("Enter a password: ");
            std::io::stdin()
                .read_line(&mut password.value)
                .expect("Problem with reading from stdio");

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
            for e in emails.into_iter() {
                println!("Email: {}", e.email);
            }
        }
        Err(error) => log::error!("Fast Mail connection failed: {:?}", error),
    }
}
