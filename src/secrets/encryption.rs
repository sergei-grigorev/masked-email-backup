use std::ops::{Deref, DerefMut};

use aes_gcm::{
    aead::{consts::U16, AeadCore, KeyInit, Nonce},
    AeadInPlace, Aes256Gcm, Key, Tag,
};
use argon2::Argon2;
use rand_core::{OsRng, RngCore};
use thiserror::Error;

use super::{AesKeyValue, PasswordValue};

pub const NONCE_SIZE_BYTES: usize = 96 / 8;

pub type KeyDerivationSalt = [u8; NONCE_SIZE_BYTES];

pub type EncryptionTag = Tag<U16>;

pub type EncryptionNonce = Nonce<Aes256Gcm>;

pub type Result<T> = std::result::Result<T, EncryptionError>;

#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("Problem with generating new AES key: {0}")]
    KeyGeneration(String),
    #[error("Encryption failed: {0}")]
    Encryption(String),
    #[error("Decryption failed: {0}")]
    Decryption(String),
}

/// Generate new salt for key derivations.
pub fn generate_new_salt() -> KeyDerivationSalt {
    let mut key_derivation_salt = [0u8; NONCE_SIZE_BYTES];
    OsRng.fill_bytes(&mut key_derivation_salt);
    key_derivation_salt
}

/// Generate new AES key from the user email and password + salt.
pub fn generate_key(
    password: &PasswordValue,
    key_derivation_salt: &KeyDerivationSalt,
) -> Result<AesKeyValue> {
    // generate 256 bits key
    let mut output_key_material = AesKeyValue::default();

    // Argon2 with default params (Argon2id v19)
    let alg = Argon2::default();

    // generate new AES key
    alg.hash_password_into(
        password.value.as_bytes(),
        key_derivation_salt,
        output_key_material.deref_mut(),
    )
    .map_err(|e| EncryptionError::KeyGeneration(e.to_string()))?;

    Ok(output_key_material)
}

/// Encrypt the buffer (in-place).
pub fn encrypt_in_place(
    key: &AesKeyValue,
    associated_data: &[u8],
    buffer: &mut [u8],
) -> Result<(EncryptionTag, EncryptionNonce)> {
    let key: &Key<Aes256Gcm> = key.deref().into();

    // generate cipher and encrypt the message
    let cipher: Aes256Gcm = Aes256Gcm::new(key);
    let nonce: Nonce<Aes256Gcm> = Aes256Gcm::generate_nonce(OsRng);
    let tag: Tag<U16> = cipher
        .encrypt_in_place_detached(&nonce, associated_data, buffer)
        .map_err(|e| EncryptionError::Encryption(e.to_string()))?;
    Ok((tag, nonce))
}

/// Descrypt the buffer (in-place).
pub fn decrypt_in_place(
    key: &AesKeyValue,
    nonce: &EncryptionNonce,
    associated_data: &[u8],
    buffer: &mut [u8],
    tag: &EncryptionTag,
) -> Result<()> {
    let key: &Key<Aes256Gcm> = key.deref().into();

    // generate cipher and encrypt the message
    let cipher: Aes256Gcm = Aes256Gcm::new(key);
    cipher.decrypt_in_place_detached(nonce, associated_data, buffer, tag).map_err(|e| EncryptionError::Decryption(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::secrets::PasswordValue;

    use super::{decrypt_in_place, encrypt_in_place, generate_key, generate_new_salt};

    #[test]
    fn decrypt_should_restore_the_content() {
        // generate a new key
        let key = generate_key(
            &PasswordValue::from("MY_WEAK_PASSWORD".to_owned()),
            &generate_new_salt(),
        )
        .expect("AES key generation failed");

        // both original message and associated data
        let original_message: String = "Hello world!".to_owned();
        let associated_data: Vec<u8> =
            Vec::<u8>::from(format!("The message length: {}", original_message.len()).as_bytes());

        // make a new buffer with the original data
        let mut buffer = Vec::<u8>::from(original_message.as_bytes());

        let (tag, salt) =
            encrypt_in_place(&key, &associated_data, &mut buffer).expect("Encryption failed");

        // decrypt and validate
        decrypt_in_place(&key, &salt, &associated_data, &mut buffer, &tag)
            .expect("Decryption failed");

        // transform to the text back
        let decrypted_message: String = String::from_utf8(buffer).expect("Message was corrupted");

        assert_eq!(original_message, decrypted_message);
    }
}
