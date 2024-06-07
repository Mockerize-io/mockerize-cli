use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub enum ResponseType {
    #[default]
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "json")]
    Json,
}
