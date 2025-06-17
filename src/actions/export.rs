use std::{fs, path::Path};

use log::{debug, info};
use mlua::{Function, Lua};
use thiserror::Error;

use crate::model::masked_email::MaskedEmail;

#[derive(Error, Debug)]
pub enum LuaError {
    #[error("File read error: {0}")]
    FileRead(#[from] std::io::Error),
    #[error("Lua script failed: {0}")]
    Lua(#[from] mlua::Error),
}

pub type Result<T> = std::result::Result<T, LuaError>;

pub fn export_lua(emails: &[MaskedEmail], script: &Path) -> Result<()> {
    let script_file = fs::read_to_string(script)?;

    // lua interpreter
    let lua = Lua::new();
    let globals = lua.globals();

    // run initialization
    info!("Loading lua script from ${script:#?}");
    lua.load(script_file).set_name("export script").exec()?;

    // table -> ()
    let prepare: Function = globals.get("prepare".to_owned())?;

    // number -> String
    let header: Function = globals.get("header".to_owned())?;

    // table -> String
    let next: Function = globals.get("next".to_owned())?;

    // () -> String
    let footer: Function = globals.get("footer".to_owned())?;

    info!("Lua script loaded successfully");

    // create struct describing format
    {
        let format = lua.create_table()?;
        format.set("internal_id", "string")?;
        format.set("email", "string")?;
        format.set("description", "string")?;
        format.set("web_site", "string")?;
        format.set("integration_url", "string")?;
        format.set("state", "string")?;
        format.set("created_at", "string")?;
        format.set("last_message_at", "string")?;

        let _ = prepare.call(format)?;
    }

    // write the header
    let mut output = String::with_capacity(1000);
    let footer_text: String = header.call(emails.len())?;
    debug!("Header has length: {}", footer_text.len());
    output.push_str(&footer_text);

    let result: std::result::Result<usize, mlua::Error> =
        emails.iter().try_fold(0usize, |acc, e| {
            let record = lua.create_table()?;
            record.set("internal_id", e.internal_id.clone())?;
            record.set("email", e.email.clone())?;
            record.set("description", e.description.clone())?;
            record.set("web_site", e.web_site.clone())?;
            record.set("integration_url", e.integration_url.clone())?;
            record.set("state", e.state.to_string())?;
            record.set("created_at", e.created_at.to_rfc2822())?;

            if let Some(last_message_at) = e.last_message_at {
                record.set("last_message_at", last_message_at.to_rfc2822())?;
            }

            let record_text: String = next.call(record)?;
            output.push_str(&record_text);
            Ok(acc + record_text.len())
        });

    // check result is ok
    let body_size = result?;
    debug!("Body has length: {body_size}");

    // write the footer
    let footer_text: String = footer.call(())?;
    debug!("Footer has length: {}", footer_text.len());
    output.push_str(&footer_text);

    // print everything to output
    println!("{}", output);

    Ok(())
}
