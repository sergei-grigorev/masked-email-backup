use std::ops::{Deref, DerefMut};

use aes_gcm::{
    aead::{consts::U16, AeadCore, KeyInit, Nonce},
    AeadInPlace, Aes256Gcm, Key, Tag,
};
use argon2::Argon2;
use rand_core::{OsRng, RngCore};

use super::{AesKeyValue, PasswordValue};

const PASSWORD_SALT_SIZE_BYTES: usize = 96 / 8;
const ENCRYPTION_SALT_SIZE_BYTES: usize = 12;

pub type KeyDerivationSalt = [u8; PASSWORD_SALT_SIZE_BYTES];

/// Generate new salt for key derivations.
pub fn generate_new_salt() -> KeyDerivationSalt {
    let mut key_derivation_salt = [0u8; PASSWORD_SALT_SIZE_BYTES];
    OsRng.fill_bytes(&mut key_derivation_salt);
    key_derivation_salt
}

/// Generate new AES key from the user email and password + salt.
pub fn generate_key(
    password: &PasswordValue,
    key_derivation_salt: &KeyDerivationSalt,
) -> Result<AesKeyValue, argon2::Error> {
    // generate 256 bits key
    let mut output_key_material = AesKeyValue::default();

    // Argon2 with default params (Argon2id v19)
    let alg = Argon2::default();

    // generate new AES key
    alg.hash_password_into(
        password.value.as_bytes(),
        key_derivation_salt,
        output_key_material.deref_mut(),
    )?;

    Ok(output_key_material)
}

/// Encrypt the buffer (in-place).
pub fn encrypt_in_place(
    key: &AesKeyValue,
    associated_data: &Vec<u8>,
    buffer: &mut Vec<u8>,
) -> Result<(Tag<U16>, Nonce<Aes256Gcm>), aes_gcm::Error> {
    let key: &Key<Aes256Gcm> = key.deref().into();

    // generate cipher and encrypt the message
    let cipher: Aes256Gcm = Aes256Gcm::new(&key);
    let nonce: Nonce<Aes256Gcm> = Aes256Gcm::generate_nonce(OsRng);
    let tag: Tag<U16> = cipher.encrypt_in_place_detached(&nonce, associated_data, buffer)?;
    Ok((tag, nonce))
}

/// Descrypt the buffer (in-place).
pub fn decrypt_in_place(
    key: &AesKeyValue,
    nonce: &Nonce<Aes256Gcm>,
    associated_data: &Vec<u8>,
    buffer: &mut Vec<u8>,
    tag: &Tag<U16>,
) -> Result<(), aes_gcm::Error> {
    let key: &Key<Aes256Gcm> = key.deref().into();

    // generate cipher and encrypt the message
    let cipher: Aes256Gcm = Aes256Gcm::new(&key);
    cipher.decrypt_in_place_detached(&nonce, associated_data, buffer, tag)?;
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
