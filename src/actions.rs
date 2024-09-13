use std::path::Path;

use clap::error;
use export::LuaError;
use thiserror::Error;

use crate::{
    config::AppConfig,
    db::disk::{DBError, Database},
    fastmail::{FastMailClient, FastMailError},
    secrets::{
        encryption::{generate_key, generate_new_salt, EncryptionError},
        fastmail::{FastMailAccount, PasswordStorageError, SecureStorage},
        AesKeyValue, PasswordValue,
    },
};

mod export;
mod show_emails;

#[derive(Error, Debug)]
pub enum ActionError {
    #[error("Problem with Password Storage: {0}")]
    PasswordStorage(#[from] PasswordStorageError),
    #[error("AES key is not found in the keychain")]
    PasswordNotFound,
    #[error("Fastmail password was not setup, please run `masked-email-cli update_password` to create a new one")]
    PasswordSetup,
    #[error("Encryption failure: {0}")]
    KeyNotGenerated(#[from] EncryptionError),
    #[error("FastMail service failed: {0}")]
    FastMail(#[from] FastMailError),
    #[error("Database error: {0}")]
    Database(#[from] DBError),
    #[error("Database not found in the path: {0}")]
    DatabaseNotFound(String),
    #[error("Lua script failed: {0}")]
    ExportScript(#[from] LuaError),
}

pub type Result<T> = std::result::Result<T, ActionError>;

pub fn refresh_db<PasswordStorage>(config: &AppConfig) -> Result<()>
where
    PasswordStorage: SecureStorage,
{
    // load token
    let account: Option<FastMailAccount> = PasswordStorage::load_password(&config.user_name)?;

    // no problems with keychain but the password is not set up
    let account: FastMailAccount = account.ok_or(ActionError::PasswordSetup)?;

    // load all masked emails
    let emails = FastMailClient::new(PasswordValue {
        value: account.bearer.value.clone(),
    })
    .and_then(|client| client.load_emails())?;

    let db: Database;
    let key: AesKeyValue;

    if let Ok(existed) = Database::init(&config.storage) {
        db = existed;

        // try to load AES key
        let existed_key = PasswordStorage::load_key(&config.user_name)?;

        match existed_key {
            Some(preloaded) => key = preloaded,
            None => {
                // if no key is stored then the key will be derived from user password and database salt
                key = generate_key(&account.into(), &db.key_derivation_salt)?;
                PasswordStorage::update_key(&config.user_name, &key)?
            }
        }
    } else {
        log::warn!("Database does not exist or cannot be decrypted. New key will be generated");

        // init new database (no files are created at this moment)
        db = Database::new(&config.storage, generate_new_salt());

        // make new AES key
        key = generate_key(&account.into(), &db.key_derivation_salt)?;

        PasswordStorage::update_key(&config.user_name, &key)?
    }

    // update the database
    db.store(&emails, &key)?;
    Ok(())
}

pub fn export_lua<PasswordStorage>(config: &AppConfig, script: &Path) -> Result<()>
where
    PasswordStorage: SecureStorage,
{
    if let Ok(db) = Database::init(&config.storage) {
        // try to load AES key
        let existed_key = PasswordStorage::load_key(&config.user_name)?;
        // todo: add key derivation here
        let existed_key = existed_key.ok_or(ActionError::PasswordNotFound)?;

        let emails = db.load(&existed_key)?;

        export::export_lua(&emails, script)?;
        Ok(())
    } else {
        Err(ActionError::DatabaseNotFound(config.storage.to_owned()))
    }
}

pub fn show_emails<PasswordStorage>(config: &AppConfig) -> Result<()>
where
    PasswordStorage: SecureStorage,
{
    if let Ok(db) = Database::init(&config.storage) {
        // try to load AES key
        let existed_key = PasswordStorage::load_key(&config.user_name)?;
        // todo: add key derivation here
        let existed_key = existed_key.ok_or(ActionError::PasswordNotFound)?;

        let emails = db.load(&existed_key)?;
        show_emails::interact(&emails);
        Ok(())
    } else {
        Err(ActionError::DatabaseNotFound(config.storage.to_owned()))
    }
}
