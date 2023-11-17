use serde::Deserialize;
// use std::time::Duration;

#[derive(Deserialize)]
pub struct MaskedEmailGet {
    #[serde(rename(deserialize = "accountId"))]
    pub account_id: String,
    pub list: Vec<MaskedEmail>,
}

#[derive(Deserialize)]
pub struct MaskedEmailSet {
    #[serde(rename(deserialize = "accountId"))]
    pub account_id: String,
    pub create: String,
}
#[derive(Deserialize, Debug)]
pub enum MaskedEmailState {
    #[serde(rename(deserialize = "pending"))]
    Pendind,
    #[serde(rename(deserialize = "enabled"))]
    Enabled,
    #[serde(rename(deserialize = "disabled"))]
    Disabled,
    #[serde(rename(deserialize = "deleted"))]
    Deleted,
}

#[derive(Deserialize)]
pub struct MaskedEmail {
    pub id: String,
    pub email: String,
    pub description: Option<String>,
    #[serde(rename(deserialize = "forDomain"))]
    pub for_domain: Option<String>,
    pub url: Option<String>,
    pub state: MaskedEmailState,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: String,
    #[serde(rename(deserialize = "lastMessageAt"))]
    pub last_message_at: Option<String>,
}
