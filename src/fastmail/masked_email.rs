use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize)]
pub enum MaskedEmailState {
    Pendind,
    Enabled,
    Disabled,
    Deleted,
}

#[derive(Deserialize)]
pub struct MaskedEmail {
    pub id: String,
    pub email: String,
    pub description: Option<String>,
    pub for_domain: Option<String>,
    pub url: Option<String>,
    pub state: MaskedEmailState,
    pub created_at: Duration,
    pub last_message_at: Option<Duration>,
}
