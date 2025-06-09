use serde::{Deserialize, Serialize};

use super::{YiweiCreative, YiweiTracker};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct YiweiAd {
    pub ad_id: String,
    pub imp_id: String,
    pub slot_id: String,
    pub creative: YiweiCreative,
    pub interaction_type: i32,
    pub price_type: i32,
    pub price: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iapp_filter: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adv_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adv_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deep_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trackers: Option<Vec<YiweiTracker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<String>,
}
