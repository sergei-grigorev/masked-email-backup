use serde::Deserialize;

#[derive(Deserialize)]
pub struct PrimaryAccounts {
    #[serde(rename(deserialize = "urn:ietf:params:jmap:core"))]
    pub account: String,
}

#[derive(Deserialize)]
pub struct SessionResponse {
    #[serde(rename(deserialize = "primaryAccounts"))]
    pub primary_accounts: PrimaryAccounts,
    #[serde(rename(deserialize = "apiUrl"))]
    pub api_url: String,
}
