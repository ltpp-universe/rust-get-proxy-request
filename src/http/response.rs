use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;
use std::collections::HashMap;
const DEFAULT_STATUS: u16 = 0;

#[derive(Serialize, Debug)]
pub struct Response {
    pub status: u16,
    pub url: String,
    pub request_header: HashMap<String, String>,
    pub response: String,
    pub response_header: HashMap<String, String>,
    pub time: String,
}

impl Response {
    pub fn default() -> Self {
        Response {
            status: DEFAULT_STATUS,
            url: String::new(),
            request_header: HashMap::new(),
            response: String::new(),
            response_header: HashMap::new(),
            time: String::new(),
        }
    }
}
