use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct Response {
    pub status: u16,
    pub url: String,
    pub request_header: HashMap<String, String>,
    pub response: String,
    pub response_header: HashMap<String, String>,
    pub time: String,
}
