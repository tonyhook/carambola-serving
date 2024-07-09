use serde::{Deserialize, Serialize};

use super::Ad;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Bid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub item: String,
    pub price: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burl: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lurl: Option<Vec<String>>,
    pub media: Ad,
}
