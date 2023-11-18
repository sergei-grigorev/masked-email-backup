use zeroize::{Zeroize, ZeroizeOnDrop};

pub mod fake;
pub mod fastmail;
pub mod keychain;

/// Struct to store passwords that memory will be always zeroize.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct PasswordValue {
    pub value: String,
}
