use zeroize::{Zeroize, ZeroizeOnDrop};

pub mod keychain;
pub mod fastmail;
pub mod fake;

/// Struct to store passwords that memory will be always zeroize.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct PasswordValue {
    pub value: String,
}
