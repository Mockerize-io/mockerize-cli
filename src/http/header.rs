use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Header {
    pub id: Uuid,
    pub key: String,
    pub value: String,
    pub active: bool,
}

impl Header {
    #[allow(unused)]
    pub fn new(key: &str, value: &str) -> Self {
        Header {
            key: key.to_string(),
            value: value.to_string(),
            id: Uuid::new_v4(),
            active: true,
        }
    }
}
