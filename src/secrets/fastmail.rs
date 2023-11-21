use std::fmt::Display;

use super::PasswordValue;

/// Network connection information.
pub struct FastMailAccount {
    pub bearer: PasswordValue,
}

impl From<FastMailAccount> for PasswordValue {
    fn from(value: FastMailAccount) -> Self {
        value.bearer
    }
}

#[derive(Debug)]
pub struct PasswordStorageError(pub String);

impl Display for PasswordStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Password storage problem: {}", self.0)
    }
}

pub type Result<A> = std::result::Result<A, PasswordStorageError>;

pub trait SecureStorage {
    /// Store password in Apple KeyChain.
    ///
    /// # Arguments
    ///
    /// * `username` - fastmail user email that is will be used to store in keychain.
    ///
    /// # Returns
    ///
    /// nothing in case the operion finished successfully
    fn update(username: &str, bearer: &PasswordValue) -> Result<()>;

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
    fn load(username: &str) -> Result<Option<FastMailAccount>>;
}
