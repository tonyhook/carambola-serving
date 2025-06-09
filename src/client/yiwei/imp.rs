use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct YiweiImp {
    pub imp_id: String,
    pub slot_id: String,
    pub ad_type: i32,
    pub ad_count: i32,
    pub is_test: bool,
    pub bid_floor: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_floor_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_min_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_max_duration: Option<i32>,
    pub creative_types: Vec<i32>,
    pub width: i32,
    pub height: i32,
    pub interaction_types: Vec<i32>,
    pub accept_price_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supports_302: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supports_ctr_agent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_support: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_bundles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_categories: Option<Vec<String>>,
}
