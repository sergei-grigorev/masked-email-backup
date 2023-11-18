use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::model::masked_email::{MaskedEmail, MaskedEmailState};

#[derive(Deserialize)]
pub struct MaskedEmailGet {
    #[serde(rename(deserialize = "accountId"))]
    pub account_id: String,
    pub list: Vec<MaskedEmailJson>,
}

#[derive(Deserialize)]
pub struct MaskedEmailSet {
    #[serde(rename(deserialize = "accountId"))]
    pub account_id: String,
    pub create: String,
}
#[derive(Deserialize, Debug)]
pub enum MaskedEmailStateJson {
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
pub struct MaskedEmailJson {
    pub id: String,
    pub email: String,
    pub description: Option<String>,
    #[serde(rename(deserialize = "forDomain"))]
    pub for_domain: Option<String>,
    pub url: Option<String>,
    pub state: MaskedEmailStateJson,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: DateTime<Utc>,
    #[serde(rename(deserialize = "lastMessageAt"))]
    pub last_message_at: Option<DateTime<Utc>>,
}

impl Into<MaskedEmailState> for MaskedEmailStateJson {
    fn into(self) -> MaskedEmailState {
        match self {
            MaskedEmailStateJson::Pendind => MaskedEmailState::TemporalPending,
            MaskedEmailStateJson::Enabled => MaskedEmailState::Active,
            MaskedEmailStateJson::Disabled => MaskedEmailState::Disabled,
            MaskedEmailStateJson::Deleted => MaskedEmailState::MarkedForDeletion,
        }
    }
}

impl Into<MaskedEmail> for MaskedEmailJson {
    fn into(self) -> MaskedEmail {
        MaskedEmail {
            internal_id: self.id,
            email: self.email,
            description: self.description,
            web_site: self.for_domain,
            integration_url: self.url,
            state: self.state.into(),
            created_at: self.created_at,
            last_message_at: self.last_message_at,
        }
    }
}
