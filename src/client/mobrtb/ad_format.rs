use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct MobrtbAdFormat {
    pub ad_unit_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_js: Option<bool>,
}
