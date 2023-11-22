use config::{
    run_args, userconfig::UserConfig, AppConfig, ConfigReader, COMMAND_INIT, COMMAND_REFRESH_DB,
    COMMAND_UPDATE_PASSWORD,
};
use fastmail::FastMailClient;
use secrets::{
    encryption::{generate_key, generate_new_salt},
    fake::FakeSecret,
    fastmail::FastMailAccount,
    AesKeyValue,
};

use crate::{
    cli::{password_prompt, user_prompt},
    db::disk::Database,
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

            // load all masked emails
            match FastMailClient::new(PasswordValue {
                value: account.bearer.value.clone(),
            })
            .and_then(|client| client.load_emails())
            {
                Ok(emails) => {
                    let db: Database;
                    let key: AesKeyValue;

                    if let Ok(existed) = Database::init(&config.storage) {
                        db = existed;

                        // todo: load key from the keychain new AES key
                        key = generate_key(&account.into(), &db.key_derivation_salt)
                            .expect("Key generation failure");
                    } else {
                        // init new database (no files are created at this moment)
                        db = Database::new(&config.storage, generate_new_salt());

                        // new AES key
                        key = generate_key(&account.into(), &db.key_derivation_salt)
                            .expect("Key generation failure");
                    }

                    // update the database
                    db.store(&emails, &key)
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
