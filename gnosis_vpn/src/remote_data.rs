use anyhow::Result;
use gnosis_vpn_lib::log_output;
use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};
use std::time;
use std::time::SystemTime;
use std::vec::Vec;

#[derive(Debug)]
pub enum RemoteData {
    NotAsked,
    Fetching {
        started_at: SystemTime,
    },
    RetryFetching {
        error: CustomError,
        // in reverse order
        backoffs: Vec<time::Duration>,
        cancel_sender: crossbeam_channel::Sender<()>,
    },
    Failure {
        error: CustomError,
    },
    Success,
}

#[derive(Debug)]
pub struct CustomError {
    pub reqw_err: Option<reqwest::Error>,
    pub status: Option<reqwest::StatusCode>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug)]
pub enum Event {
    Response(serde_json::Value),
    Retry,
    Error(CustomError),
}

pub fn authentication_headers(api_token: &str) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let mut hv_token = HeaderValue::from_str(api_token)?;
    hv_token.set_sensitive(true);
    headers.insert("x-auth-token", hv_token);
    Ok(headers)
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Event::Response(val) => write!(f, "Response: {:?}", val),
            Event::Retry => write!(f, "Retry"),
            Event::Error(e) => write!(f, "Error: {:?}", e),
        }
    }
}

impl std::fmt::Display for RemoteData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RemoteData::NotAsked => write!(f, "NotAsked"),
            RemoteData::Fetching { started_at } => {
                write!(f, "Fetching since {}", log_output::elapsed(started_at))
            }
            RemoteData::RetryFetching {
                error,
                backoffs,
                cancel_sender: _,
            } => write!(f, "RetryFetching error: {:?}, backoffs: {:?}", error, backoffs),
            RemoteData::Failure { error } => write!(f, "Failure: {:?}", error),
            RemoteData::Success => write!(f, "Success"),
        }
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "CustomError: reqw_err: {:?}, status: {:?}, value: {:?}",
            self.reqw_err, self.status, self.value
        )
    }
}
