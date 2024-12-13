use serde::{Deserialize, Serialize};

use super::{FwbAdmobject, FwbAppAsset, FwbEvents};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FwbBid {
    pub id: String,
    pub adid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crid: Option<String>,
    pub impid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admobject: Option<FwbAdmobject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universal_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_app_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxappid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxapppath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<FwbAppAsset>,
    pub events: FwbEvents,
}
