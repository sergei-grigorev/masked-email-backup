use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MaskedEmailState {
    TemporalPending,
    Active,
    Disabled,
    MarkedForDeletion,
}

#[derive(Serialize, Deserialize)]
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
