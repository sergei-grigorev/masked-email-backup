use serde::{Deserialize, Serialize};
use std::io::{BufReader, BufWriter};
use std::u32;
use std::{fs, path::PathBuf};
use thiserror::Error;

use chrono::{DateTime, Utc};

use crate::secrets::encryption::{
    decrypt_in_place, encrypt_in_place, EncryptionNonce, EncryptionTag, KeyDerivationSalt,
    NONCE_SIZE_BYTES,
};

const DATABASE_FILE_NAME: &'static str = "masked_mails.db.enc";
const FILE_SIGNATURE: [u8; 4] = [b'M', b'E', b'F', 1u8];

/// File format specification:
/// ### first 28 bytes are preloaded
/// - file signature 4 bytes
/// - AES key nonce 12 bytes
/// - last updated TS (8 bytes)
/// - records count (4 bytes)
/// ### others are not a part of the preloaded header
/// - unique nonce 12 bytes
/// - tag 16 bytes (from last updated + records count)
/// - total encrypted block bytes length (8 bytes)
/// - encrypted block (see the size above)
pub struct Database {
    path: PathBuf,
    pub key_derivation_salt: KeyDerivationSalt,
    pub last_update: DateTime<Utc>,
    pub records_count: u32,
}

#[derive(Debug, Error)]
pub enum DBError {
    #[error("storage is not found ({0})")]
    FileNotFound(String),
    #[error("file has an incorrect format")]
    IncorrectFileFormat,
    #[error("encoding error")]
    EncodingError,
    #[error("decoding error")]
    DecodingError,
    #[error("Disk IO error")]
    IOError(#[from] std::io::Error),
}

#[derive(Deserialize, Serialize)]
struct FileHeader {
    file_signature: [u8; 4],
    nonce: [u8; NONCE_SIZE_BYTES],
    last_updated: DateTime<Utc>,
    records_count: u32,
}

pub type Result<A> = std::result::Result<A, DBError>;

impl Database {
    /// Initialize object. That will read the file but not decrypt it. The function checks the database file exists
    /// and contains the right file format. It validate the file has the supported format.
    pub fn init<P: Into<std::path::PathBuf>>(path: P) -> Result<Self> {
        let mut full_path: PathBuf = path.into();
        full_path.push(DATABASE_FILE_NAME);
        if full_path.exists() {
            match fs::read(full_path.as_path()) {
                Ok(_) => {
                    // read file header only
                    let file =
                        fs::File::open(full_path.as_path()).map_err(|e| DBError::IOError(e))?;
                    let buffer = BufReader::new(file);

                    // deserialize file header
                    let header: bincode::Result<FileHeader> = bincode::deserialize_from(buffer);
                    match header {
                        Ok(header) => {
                            if header.file_signature == FILE_SIGNATURE {
                                // parse last updated_ts and records count
                                Ok(Database {
                                    path: full_path.into(),
                                    key_derivation_salt: header.nonce,
                                    last_update: header.last_updated,
                                    records_count: header.records_count,
                                })
                            } else {
                                Err(DBError::IncorrectFileFormat)
                            }
                        }
                        Err(e) => {
                            log::error!("File header cannot be deserialized: {:?}", e.as_ref());
                            Err(DBError::IncorrectFileFormat)
                        }
                    }
                }
                Err(err) => {
                    log::error!(
                        "Problem reading file [{:?}]: {:?}",
                        full_path.as_path(),
                        err
                    );
                    Err(DBError::IOError(err))
                }
            }
        } else {
            Err(DBError::FileNotFound(
                full_path.to_str().unwrap_or_default().to_owned(),
            ))
        }
    }

    /// Init new database.
    pub fn new<P: Into<PathBuf>>(path: P, nonce: KeyDerivationSalt) -> Self {
        let mut full_path: PathBuf = path.into();
        full_path.push(DATABASE_FILE_NAME);

        Database {
            path: full_path,
            key_derivation_salt: nonce,
            last_update: Utc::now(),
            records_count: 0,
        }
    }

    /// load the database and all emails that it has. email and password are used to derive the encryption key.
    ///
    /// # arguments
    ///
    /// * `key` - aes encryption key
    pub fn load(
        &self,
        key: &crate::secrets::AesKeyValue,
    ) -> Result<Vec<crate::model::masked_email::MaskedEmail>> {
        use std::io::Read;

        if self.records_count > 0 {
            // read file header only
            let file = fs::File::open(self.path.as_path()).map_err(|e| DBError::IOError(e))?;
            let mut buffer = BufReader::new(file);

            // skip file header
            let header: bincode::Result<FileHeader> = bincode::deserialize_from(&mut buffer);
            if let Ok(file_header) = header {
                let associated_data =
                    bincode::serialize(&(file_header.last_updated, file_header.records_count))
                        .expect("Error is not expected");

                // - unique nonce 12 bytes
                let mut nonce: EncryptionNonce = Default::default();
                buffer
                    .read_exact(&mut nonce)
                    .map_err(|ioe| DBError::IOError(ioe))?;

                // - tag 16 bytes
                let mut tag: EncryptionTag = Default::default();
                buffer
                    .read_exact(&mut tag)
                    .map_err(|ioe| DBError::IOError(ioe))?;

                // - total encrypted block bytes length (8 bytes)
                let block_size: u64 =
                    bincode::deserialize_from(&mut buffer).map_err(|_| DBError::DecodingError)?;
                let block_size =
                    usize::try_from(block_size).expect("File size is too big for that platform");

                // - encrypted block (see the size above)
                let mut encrypted_blob: Vec<u8> = Vec::<u8>::with_capacity(block_size);
                buffer
                    .read_to_end(&mut encrypted_blob)
                    .map_err(|ioe| DBError::IOError(ioe))?;
                if encrypted_blob.len() == block_size {
                    // decrypt the blob
                    decrypt_in_place(&key, &nonce, &associated_data, &mut encrypted_blob, &tag)
                        .map_err(|_| DBError::DecodingError)?;

                    // transform to emails
                    bincode::deserialize(&encrypted_blob).map_err(|_| DBError::DecodingError)
                } else {
                    Err(DBError::IncorrectFileFormat)
                }
            } else {
                Err(DBError::DecodingError)
            }
        } else {
            Ok(Vec::new())
        }
    }

    /// Update the database and store the new email list. It generates the database and encrypts all emails.
    ///
    /// # Arguments
    ///
    /// * `emails` - masked emails
    /// * `key` - AES encryption key
    pub fn store(
        &self,
        emails: &Vec<crate::model::masked_email::MaskedEmail>,
        aes: &crate::secrets::AesKeyValue,
    ) -> Result<()> {
        use std::io::Write;

        // create the root directory if that doesn't exist
        if let Some(root) = self.path.parent() {
            std::fs::create_dir_all(root).map_err(|e| DBError::IOError(e))?;
        }

        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.path.as_path())
            .map_err(|e| DBError::IOError(e))?;

        let mut buffer = BufWriter::new(file);

        let file_header = FileHeader {
            file_signature: FILE_SIGNATURE,
            nonce: self.key_derivation_salt,
            last_updated: Utc::now(),
            records_count: u32::try_from(emails.len()).expect("Arrays is too big"),
        };

        // serialize header
        bincode::serialize_into(&mut buffer, &file_header).expect("Error is not expected");

        // encode emails list
        let mut content_buffer = bincode::serialize(&emails).map_err(|_| DBError::EncodingError)?;

        // encrypt that block
        let associated_data =
            bincode::serialize(&(file_header.last_updated, file_header.records_count))
                .expect("No error expected");
        let (tag, nonce) = encrypt_in_place(&aes, &associated_data, &mut content_buffer)
            .map_err(|_| DBError::EncodingError)?;

        // serialize nonce
        buffer.write(&nonce).map_err(|ioe| DBError::IOError(ioe))?;
        // serialize tag
        buffer.write(&tag).map_err(|ioe| DBError::IOError(ioe))?;

        // serialize binary length
        let blob_size: u64 = content_buffer
            .len()
            .try_into()
            .expect("Blob size is too big");
        bincode::serialize_into(&mut buffer, &blob_size).map_err(|_| DBError::EncodingError)?;

        // serialize binary
        buffer
            .write(&content_buffer)
            .map_err(|ioe| DBError::IOError(ioe))?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{fs, path::PathBuf};

    use chrono::Utc;

    use crate::{
        db::disk::{FileHeader, FILE_SIGNATURE},
        model::masked_email::{MaskedEmail, MaskedEmailState},
        secrets::{
            encryption::{generate_key, generate_new_salt},
            PasswordValue,
        },
    };

    use super::{DBError, Database, DATABASE_FILE_NAME};

    #[test]
    fn database_not_exists() {
        // make new tmp directory
        let tmp_dir = tempfile::tempdir().unwrap();
        let res = Database::init(tmp_dir.path());
        match res {
            Err(DBError::FileNotFound(_)) => (),
            _ => panic!("Incorrect behaviour for database not found"),
        }
    }

    #[test]
    fn incorrect_file_format() {
        // make new tmp directory
        let tmp_dir = tempfile::tempdir().unwrap();
        let mut tmp_file: PathBuf = PathBuf::from(tmp_dir.path());
        tmp_file.push(DATABASE_FILE_NAME);

        // make a new just a random text file
        fs::write(tmp_file, "Masked Email Database").expect("IO Error");

        // try to read database
        let res = Database::init(tmp_dir.path());
        match res {
            Err(DBError::IncorrectFileFormat) => (),
            _ => panic!("Incorrect behaviour for database file in incorrect format"),
        }
    }

    #[test]
    fn read_valid_file_signature() {
        // make new tmp directory
        let tmp_dir = tempfile::tempdir().unwrap();
        let mut tmp_file: PathBuf = PathBuf::from(tmp_dir.path());
        tmp_file.push(DATABASE_FILE_NAME);

        let file_header = FileHeader {
            file_signature: FILE_SIGNATURE,
            nonce: generate_new_salt(),
            last_updated: Utc::now(),
            records_count: 100u32,
        };

        let content = bincode::serialize(&file_header).unwrap();
        fs::write(tmp_file, content).expect("IO Error");

        // try to read database
        let res = Database::init(tmp_dir.path()).expect("Error reading file");
        assert_eq!(res.key_derivation_salt, file_header.nonce);
        assert_eq!(res.last_update, file_header.last_updated);
        assert_eq!(res.records_count, file_header.records_count);
    }

    #[test]
    fn save_new_and_reload() {
        // make new tmp directory
        let tmp_dir = tempfile::tempdir().unwrap();

        // init new cipher
        let db1 = Database::new(tmp_dir.path(), generate_new_salt());
        assert_eq!(db1.records_count, 0u32);
        assert_eq!(db1.path.parent().unwrap(), tmp_dir.path());

        let records = vec![MaskedEmail {
            internal_id: "id1".to_owned(),
            email: "example@example.com".to_owned(),
            description: None,
            web_site: None,
            integration_url: None,
            state: MaskedEmailState::Active,
            created_at: Default::default(),
            last_message_at: None,
        }];

        // make new AES
        let key = generate_key(
            &PasswordValue {
                value: "weak_password".to_owned(),
            },
            &db1.key_derivation_salt,
        )
        .expect("AES generation failed");

        // save database
        db1.store(&records, &key).expect("Serialization failed");

        // try to read database
        let db2 = Database::init(tmp_dir.path()).expect("Failed to open the file");
        assert_eq!(db2.path, db1.path);
        assert_eq!(
            TryInto::<usize>::try_into(db2.records_count).unwrap(),
            records.len()
        );
        assert!(db2.last_update < Utc::now());
        assert_eq!(db2.key_derivation_salt, db1.key_derivation_salt);

        let res = db2.load(&key).expect("Decryption failed");

        // validate everything has been properly decrypted
        assert_eq!(res, records);
    }
}
