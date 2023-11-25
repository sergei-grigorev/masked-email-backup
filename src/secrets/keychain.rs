use super::{
    fastmail::{FastMailAccount, PasswordStorageError, Result, SecureStorage},
    AESKey, AesKeyValue, PasswordValue,
};
use base64::{engine::general_purpose, Engine};
use core_foundation::base::OSStatus;
use security_framework::{
    base::Error,
    passwords::{delete_generic_password, get_generic_password, set_generic_password},
};

const FASTMAIL_SERVICE_NAME: &str = "fast-mail-cli";
const AES_SERVICE_NAME: &str = "fast-mail-cli-aes";
const NOT_FOUND_CODE: OSStatus = -25300;

impl From<Error> for PasswordStorageError {
    fn from(value: Error) -> Self {
        PasswordStorageError(value.to_string())
    }
}

pub struct KeyChain();

impl KeyChain {
    /// Store password in Apple KeyChain.
    ///
    /// # Arguments
    ///
    /// * `service` - service name
    /// * `username` - fastmail user email that is will be used to store in keychain.
    ///
    /// # Returns
    ///
    /// nothing in case the operion finished successfully
    fn update_password(service: &str, username: &str, bearer: &PasswordValue) -> Result<()> {
        // create a new password
        if let Err(e) = delete_generic_password(service, username) {
            log::warn!(
                "Old password was not deleted (if that exists): {}",
                e.message().unwrap_or_default()
            );
        }

        // create a new password
        set_generic_password(service, username, bearer.value.as_bytes())
            .map_err(PasswordStorageError::from)?;
        log::info!(
            "New password was stored in KeyChain: [{}] / [{}]",
            FASTMAIL_SERVICE_NAME,
            username
        );

        Ok(())
    }

    /// Load password from the Apple KeyChain.
    ///
    /// # Arguments
    ///
    /// * `service` - service name
    /// * `username` - fastmail user email that is used to store in keychain
    ///
    /// # Returns
    ///
    /// empty in case of no user found. Otherwise it will be a sucessful result.
    ///
    fn load_password(service: &str, username: &str) -> Result<Option<PasswordValue>> {
        match get_generic_password(service, username) {
            Ok(password) => {
                let token = String::from_utf8(password).expect("Password has incorrect symbols");
                Ok(Some(PasswordValue { value: token }))
            }
            Err(e) => {
                if e.code() == NOT_FOUND_CODE {
                    log::info!("Password was not found");
                    Ok(None)
                } else {
                    log::warn!(
                        "Password access was forbidden: {}",
                        e.message().unwrap_or_default()
                    );
                    Err(e.into())
                }
            }
        }
    }
}

impl SecureStorage for KeyChain {
    fn update_password(username: &str, bearer: &PasswordValue) -> Result<()> {
        KeyChain::update_password(FASTMAIL_SERVICE_NAME, username, bearer)
    }

    fn load_password(username: &str) -> Result<Option<FastMailAccount>> {
        KeyChain::load_password(FASTMAIL_SERVICE_NAME, username)
            .map(|maybe_pass| maybe_pass.map(|p| FastMailAccount { bearer: p }))
    }

    fn load_key(username: &str) -> Result<Option<super::AesKeyValue>> {
        KeyChain::load_password(AES_SERVICE_NAME, username).map(|maybe_pass| {
            maybe_pass.and_then(|base64| {
                #[allow(clippy::needless_borrows_for_generic_args)]
                if let Ok(vec) = general_purpose::STANDARD.decode(&base64.value) {
                    if let Ok(aes) = AESKey::try_from(vec) {
                        Some(AesKeyValue { value: aes })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
    }

    fn update_key(username: &str, aes: &super::AesKeyValue) -> Result<()> {
        let encoded = general_purpose::STANDARD.encode(aes.value);
        let secure_string = PasswordValue { value: encoded };
        KeyChain::update_password(AES_SERVICE_NAME, username, &secure_string)
    }
}
