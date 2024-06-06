use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Header, ResponseType};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: Uuid,
    pub name: String,
    pub status: u16,
    response: String,
    #[serde(rename = "responseType")]
    pub response_type: ResponseType,
    pub active: bool,
    pub headers: Vec<Header>,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            id: Uuid::new_v4(),
            name: String::default(),
            status: 200,
            response: String::default(),
            response_type: ResponseType::default(),
            active: true,
            headers: vec![],
        }
    }
}

impl Response {
    #[allow(unused)]
    pub fn new(name: &str, status: u16, response_type: ResponseType, body: &str) -> Self {
        Response {
            name: String::from(name),
            status,
            response_type,
            response: String::from(body),
            ..Default::default()
        }
    }

    pub fn get_response_body(&self) -> String {
        self.response.clone()
    }

    #[allow(unused)]
    pub fn add_header(&mut self, header: Header) -> &mut Self {
        self.headers.push(header);
        self
    }
}
