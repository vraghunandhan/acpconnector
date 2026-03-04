use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Address {
    #[validate(length(max = 256))]
    #[serde(rename = "name")]
    pub name: String,
    #[validate(length(max = 60))]
    #[serde(rename = "line_one")]
    pub line_one: String,
    #[validate(length(max = 60))]
    #[serde(rename = "line_two")]
    pub line_two: Option<String>,
    #[validate(length(max = 60))]
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "state")]
    pub state: Option<String>,
    #[serde(rename = "country")]
    pub country: String,
    #[validate(length(max = 20))]
    #[serde(rename = "postal_code")]
    pub postal_code: String,
    #[serde(rename = "phone_number")]
    pub phone_number: Option<String>,
}
