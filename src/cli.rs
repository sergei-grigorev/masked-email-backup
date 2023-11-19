use std::io::{stdout, Write};

use crate::secrets::PasswordValue;

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
    print!("{}: ", prompt);
    stdout().flush()?;

    let mut res = String::new();
    std::io::stdin().read_line(&mut res)?;

    Ok(res.trim_end().to_owned())
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
    print!("{}: ", prompt);
    stdout().flush()?;
    let fast_mail_password = rpassword::read_password()?;
    let password = PasswordValue {
        value: fast_mail_password,
    };

    Ok(password)
}
