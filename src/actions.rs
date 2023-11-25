use crate::{
    config::AppConfig,
    db::disk::Database,
    fastmail::FastMailClient,
    secrets::{
        encryption::{generate_key, generate_new_salt},
        fastmail::{FastMailAccount, SecureStorage},
        AesKeyValue, PasswordValue,
    },
};

mod export;
mod show_emails;

pub enum ExportFormat {
    Tsv,
}

pub fn refresh_db<PasswordStorage>(config: &AppConfig) -> Result<(), String>
where
    PasswordStorage: SecureStorage,
{
    // load token
    let account: Option<FastMailAccount> = PasswordStorage::load_password(&config.user_name)
        .map_err(|_| "Keychain access failed, please add permissions or recreate the password")?;

    // no problems with keychain but the password is not set up
    let account: FastMailAccount = account.ok_or("Fastmail password was not setup, please run `masked-email-cli update_password` to create a new one".to_owned())?;

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

                // try to load AES key
                let existed_key = PasswordStorage::load_key(&config.user_name).map_err(|_| {
                    "Keychain access failed, please add permissions or delete the encryption key"
                })?;

                match existed_key {
                    Some(preloaded) => key = preloaded,
                    None => {
                        // if no key is stored then the key will be derived from user password and database salt
                        key = generate_key(&account.into(), &db.key_derivation_salt)
                            .expect("Key generation failure");
                        PasswordStorage::update_key(&config.user_name, &key)
                            .map_err(|_| "Key cannot be stored to keychain")?;
                    }
                }
            } else {
                log::warn!(
                    "Database does not exist or cannot be decrypted. New key will be generated"
                );

                // init new database (no files are created at this moment)
                db = Database::new(&config.storage, generate_new_salt());

                // make new AES key
                key = generate_key(&account.into(), &db.key_derivation_salt)
                    .expect("Key generation failure");

                PasswordStorage::update_key(&config.user_name, &key)
                    .map_err(|_| "Key cannot be stored to keychain")?;
            }

            // update the database
            db.store(&emails, &key)
                .expect("Problem storing emails in the database");
            Ok(())
        }
        Err(error) => Err(format!("Fast Mail connection failed: {:?}", error)),
    }
}

pub fn export_emails<PasswordStorage>(
    config: &AppConfig,
    format: ExportFormat,
) -> Result<(), String>
where
    PasswordStorage: SecureStorage,
{
    if let Ok(db) = Database::init(&config.storage) {
        // try to load AES key
        let existed_key = PasswordStorage::load_key(&config.user_name).map_err(|_| {
            "Keychain access failed, please add permissions or delete the encryption key"
        })?;

        // todo: add key derivation here
        let existed_key = existed_key.ok_or("AES key is not found in the keychain")?;

        let emails = db.load(&existed_key).map_err(|_| "Decryption error")?;

        // todo: convert to the right format
        match format {
            ExportFormat::Tsv => {
                export::export_tsv(&emails);
            }
        }
        Ok(())
    } else {
        Err("Database is not found".to_string())
    }
}

pub fn show_emails<PasswordStorage>(config: &AppConfig) -> Result<(), String>
where
    PasswordStorage: SecureStorage,
{
    if let Ok(db) = Database::init(&config.storage) {
        // try to load AES key
        let existed_key = PasswordStorage::load_key(&config.user_name).map_err(|_| {
            "Keychain access failed, please add permissions or delete the encryption key"
        })?;

        // todo: add key derivation here
        let existed_key = existed_key.ok_or("AES key is not found in the keychain")?;

        let emails = db.load(&existed_key).map_err(|_| "Decryption error")?;
        show_emails::interact(&emails);
        Ok(())
    } else {
        Err("Database is not found".to_string())
    }
}
