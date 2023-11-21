use std::fmt::Display;

use reqwest::{blocking::Client, header, StatusCode};
use serde_json::json;

use crate::{
    fastmail::json::{
        method_response::{JMapMethodResponse, JMapResponse, MethodResponse},
        session::SessionResponse,
    },
    model::masked_email::MaskedEmail,
    secrets::PasswordValue,
};

mod json;

const SESSION_API_URL: &str = "https://api.fastmail.com/jmap/session";

#[derive(Debug)]
pub enum FastMailError {
    RequestFailed(reqwest::Error),
    RequestErrorCode(StatusCode, String),
}

impl Display for FastMailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            FastMailError::RequestFailed(error) => {
                write!(f, "Problem with the HTTP Request: {:?}", error)
            }
            FastMailError::RequestErrorCode(http_code, text) => {
                write!(
                    f,
                    "HTTP Request failed [{}]: {}",
                    http_code,
                    text.trim_end()
                )
            }
        }
    }
}

impl From<reqwest::Error> for FastMailError {
    fn from(value: reqwest::Error) -> Self {
        FastMailError::RequestFailed(value)
    }
}

pub type Result<A> = std::result::Result<A, FastMailError>;

pub struct FastMailClient {
    client: Client,
    token: PasswordValue,
    url: String,
    account: String,
}

impl FastMailClient {
    /// Create a new fast mail session. That calls fastmail session api
    /// to derive the right api sql and validate user token.
    pub fn new(token: PasswordValue) -> Result<Self> {
        let client = FastMailClient::make_client();
        let req = client.get(SESSION_API_URL).bearer_auth(&token.value);

        let resp = req.send().map_err(|e| FastMailError::from(e))?;

        if resp.status() == StatusCode::OK {
            let resp = resp
                .json::<SessionResponse>()
                .map_err(|e| FastMailError::from(e))?;
            Ok(FastMailClient {
                client,
                token,
                url: resp.api_url,
                account: resp.primary_accounts.account,
            })
        } else {
            let error_code = resp.status();
            let resp = resp.text().map_err(|e| FastMailError::from(e))?;
            Err(FastMailError::RequestErrorCode(error_code, resp))
        }
    }

    /// Load All Masked Emails.
    ///
    /// # Returns
    ///
    /// List of all current emails, including disabled.
    pub fn load_emails(&self) -> Result<Vec<MaskedEmail>> {
        let user_id: &str = self.account.as_str();
        let query_id = "a";
        let body = json!({
            "using": [ "https://www.fastmail.com/dev/maskedemail" ],
            "methodCalls": [
                ["MaskedEmail/get",
                    { "accountId": user_id},
                 query_id
                ]
            ]
        });

        log::info!("Load emails for the user: [{}]", user_id);

        let req = self
            .client
            .post(&self.url)
            .bearer_auth(&self.token.value)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&body);

        let resp = req.send().map_err(|e| FastMailError::from(e))?;

        if resp.status() == StatusCode::OK {
            let resp = resp
                .json::<JMapResponse>()
                .map_err(|e| FastMailError::from(e))?;
            let emails = resp
                .method_responses
                .into_iter()
                .filter_map(|response| {
                    if let JMapMethodResponse(_, MethodResponse::MaskedEmailGet(resp), _) = response
                    {
                        let res = resp.list.into_iter().map(|email| email.into());
                        Some(res)
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<MaskedEmail>>();

            Ok(emails)
        } else {
            let error_code = resp.status();
            let resp = resp.text().map_err(|e| FastMailError::from(e))?;
            Err(FastMailError::RequestErrorCode(error_code, resp))
        }
    }

    /// Load Masked Email.
    ///
    /// # Arguments
    ///
    /// * `id` - fastmail email ID
    ///
    /// # Returns
    ///
    /// Email or empty element if that ID is not found.
    // pub fn load_email(&self, id: &str) -> Result<Option<MaskedEmail>> {
    //     todo!()
    // }

    /// Update Masked Email.
    ///
    /// # Arguments
    ///
    /// * `masked_email` - new values for the email ID
    ///
    /// # Returns
    ///
    /// Reloaded masked email value from the server.
    // pub fn refresh(&self, masked_email: MaskedEmail) -> Result<MaskedEmail> {
    //     let reloaded = self.load_email(&masked_email.id)?;
    //     Ok(reloaded.expect("Reloading email failed"))
    // }

    #[inline]
    fn make_client() -> Client {
        reqwest::blocking::Client::new()
    }
}
