use thiserror::Error;

use super::{AesKeyValue, PasswordValue};

/// Network connection information.
pub struct FastMailAccount {
    pub bearer: PasswordValue,
}

impl From<FastMailAccount> for PasswordValue {
    fn from(value: FastMailAccount) -> Self {
        value.bearer
    }
}

#[derive(Debug, Error)]
#[error("password storage problem: {0}")]
pub struct PasswordStorageError(pub String);

pub type Result<A> = std::result::Result<A, PasswordStorageError>;

pub trait SecureStorage {
    /// Store password in KeyChain.
    ///
    /// # Arguments
    ///
    /// * `username` - fastmail user email that is will be used to store in keychain.
    ///
    /// # Returns
    ///
    /// nothing in case the operion finished successfully
    fn update_password(username: &str, bearer: &PasswordValue) -> Result<()>;

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
    fn load_password(username: &str) -> Result<Option<FastMailAccount>>;

    /// Load AES key from Keychain.
    ///
    /// # Arguments
    ///
    /// * `username` - fastmail user email that is will be used to store in keychain.
    ///
    /// # Returns
    ///
    /// nothing in case the operion finished successfully
    fn load_key(username: &str) -> Result<Option<AesKeyValue>>;

    /// Store AES key in KeyChain.
    ///
    /// # Arguments
    ///
    /// * `username` - fastmail user email that is will be used to store in keychain.
    ///
    /// # Returns
    ///
    /// nothing in case the operion finished successfully
    fn update_key(username: &str, key: &AesKeyValue) -> Result<()>;
}
