use serde::Deserialize;

use super::masked_email_get::{MaskedEmailGet, MaskedEmailSet};

#[derive(Deserialize)]
pub struct JMapResponse {
    #[serde(rename(deserialize = "methodResponses"))]
    pub method_responses: Vec<JMapMethodResponse>,
}

#[derive(Deserialize)]
pub struct JMapMethodResponse(pub String, pub MethodResponse, pub String);

#[derive(Deserialize)]
#[serde(untagged)]
pub enum MethodResponse {
    MaskedEmailGet(MaskedEmailGet),
    MaskedEmailSet(MaskedEmailSet),
}
