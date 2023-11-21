use std::{fmt::Display, path::PathBuf};

use crate::{model::masked_email::MaskedEmail, secrets::PasswordValue};

pub mod fake;

#[derive(Debug)]
pub enum DBError {
    FileNotFound(String),
    EncryptionError,
    DecryptionError,
    IOError(std::io::Error),
}

impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::FileNotFound(path) => write!(f, "Storage is not found ({})", path),
            DBError::EncryptionError => write!(f, "Problem with the encryption"),
            DBError::DecryptionError => {
                write!(f, "Problem with the decryption, try to update the password")
            }
            DBError::IOError(io_error) => write!(f, "IO Error: {}", io_error),
        }
    }
}

pub type Result<A> = std::result::Result<A, DBError>;

pub trait Database: Sized {
    /// Initialize object. That will read the file but not decrypt it. The function checks the database file exists
    /// and contains the right file format. It validate the file has the supported format.
    fn init<P: Into<PathBuf>>(path: P) -> Result<Self>;

    /// Load the database and all emails that it has. Email and password are used to derive the encryption key.
    ///
    /// # Arguments
    ///
    /// * `email` - fastmail user email
    /// * `pass` - fastmail application specific password
    fn load(&self, email: &str, pass: &PasswordValue) -> Result<Vec<MaskedEmail>>;

    /// Update the database and store the new email list. It generates the database and encrypts all emails.
    ///
    /// # Arguments
    ///
    /// * `email` - fastmail user email
    /// * `pass` - fastmail application specific password
    fn store(&self, emails: Vec<MaskedEmail>, email: &str, pass: &PasswordValue) -> Result<()>;
}
