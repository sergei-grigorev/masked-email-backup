use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MaskedEmailState {
    TemporalPending,
    Active,
    Disabled,
    MarkedForDeletion,
}

impl Display for MaskedEmailState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            MaskedEmailState::TemporalPending => write!(f, "pending"),
            MaskedEmailState::Active => write!(f, "enabled"),
            MaskedEmailState::Disabled => write!(f, "disabled"),
            MaskedEmailState::MarkedForDeletion => write!(f, "deleted"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MaskedEmail {
    pub internal_id: String,
    pub email: String,
    pub description: Option<String>,
    pub web_site: Option<String>,
    pub integration_url: Option<String>,
    pub state: MaskedEmailState,
    pub created_at: DateTime<Utc>,
    pub last_message_at: Option<DateTime<Utc>>,
}
