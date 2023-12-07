use std::ops::{Deref, DerefMut};

use zeroize::{Zeroize, ZeroizeOnDrop};

pub mod encryption;
pub mod fastmail;
pub mod keychain;

const KEY_SIZE_BYTES: usize = 32;
pub type AESKey = [u8; KEY_SIZE_BYTES];

/// Struct to store passwords that memory will be always zeroize.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct PasswordValue {
    pub value: String,
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct AesKeyValue {
    pub value: AESKey,
}

impl Default for AesKeyValue {
    fn default() -> Self {
        Self {
            value: [0u8; KEY_SIZE_BYTES],
        }
    }
}

impl Deref for AesKeyValue {
    type Target = AESKey;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for AesKeyValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl From<String> for PasswordValue {
    fn from(value: String) -> Self {
        PasswordValue { value }
    }
}
