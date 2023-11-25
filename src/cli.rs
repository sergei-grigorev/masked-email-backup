use dialoguer::{Input, Password};

use crate::secrets::PasswordValue;

fn to_io_error(error: dialoguer::Error) -> std::io::Error {
    match error {
        dialoguer::Error::IO(e) => e,
    }
}

/// Print the user prompt text and wait user input.
///
/// # Parameters
///
/// * `prompt` - text what exactly is required from the user
///
/// # Returns
///
/// truncated string that user has entered
pub fn user_prompt(prompt: &str) -> Result<String, std::io::Error> {
    let res = Input::<String>::new()
        .with_prompt(prompt)
        .interact_text()
        .map_err(to_io_error)?;
    Ok(res.to_owned())
}

/// Print the user prompt text and wait user input. In comparison with [[user_propmt]] the user input will be not printed.
///
/// # Parameters
///
/// * `prompt` - text what exactly is required from the user
///
/// # Returns
///
/// password that user has entered
pub fn password_prompt(prompt: &str) -> Result<PasswordValue, std::io::Error> {
    let fast_mail_password = Password::new()
        .with_prompt(prompt)
        .interact()
        .map_err(to_io_error)?;
    let password = PasswordValue {
        value: fast_mail_password,
    };

    Ok(password)
}
