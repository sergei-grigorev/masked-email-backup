use super::{
    fastmail::{FastMailAccount, PasswordStorageError, Result, SecureStorage},
    PasswordValue,
};
use core_foundation::base::OSStatus;
use security_framework::{
    base::Error,
    passwords::{delete_generic_password, get_generic_password, set_generic_password},
};

const SERVICE_NAME: &str = "fast-mail-cli";
const NOT_FOUND_CODE: OSStatus = -25300;

impl From<Error> for PasswordStorageError {
    fn from(value: Error) -> Self {
        PasswordStorageError(value.to_string())
    }
}

pub struct KeyChain();

impl SecureStorage for KeyChain {
    /// Store password in Apple KeyChain.
    ///
    /// # Arguments
    ///
    /// * `username` - fastmail user email that is will be used to store in keychain.
    ///
    /// # Returns
    ///
    /// nothing in case the operion finished successfully
    fn update(username: &str, bearer: &PasswordValue) -> Result<()> {
        // create a new password
        if let Err(e) = delete_generic_password(SERVICE_NAME, username) {
            log::warn!(
                "Old password was not deleted (if that exists): {}",
                e.message().unwrap_or_default()
            );
        }

        // create a new password
        set_generic_password(SERVICE_NAME, username, bearer.value.as_bytes())
            .map_err(|e| PasswordStorageError::from(e))?;
        log::info!(
            "New password was stored in KeyChain: [{}] / [{}]",
            SERVICE_NAME,
            username
        );

        Ok(())
    }

    /// Load password from the Apple KeyChain.
    ///
    /// # Arguments
    ///
    /// * `username` - fastmail user email that is used to store in keychain
    ///
    /// # Returns
    ///
    /// empty in case of no user found. Otherwise it will be a sucessful result.
    ///
    fn load(username: &str) -> Result<Option<FastMailAccount>> {
        match get_generic_password(SERVICE_NAME, &username) {
            Ok(password) => {
                let token = String::from_utf8(password).expect("Password has incorrect symbols");
                Ok(Some(FastMailAccount {
                    bearer: PasswordValue { value: token },
                }))
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
