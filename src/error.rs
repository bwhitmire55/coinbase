use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum APIError {
    #[error("Failed fetching: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Request failed: {0}")]
    BadRequest(&'static str),

    #[error("Coinbase Error: {0}")]
    CoinbaseError(String),

    #[error("Unknown response")]
    UnknownResponse
}

#[derive(Deserialize, Debug)]
pub struct CBError {
    pub id: String,
    pub message: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CBErrorResponse {
    pub errors: Vec<CBError>,
}