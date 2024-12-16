use serde::{Deserialize, Serialize};

use super::{KkmhAdm, KkmhAppinfo, KkmhMonitor};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KkmhBid {
    pub ad_id: String,
    pub imp_id: String,
    pub price: f64,
    pub adms: Vec<KkmhAdm>,
    pub click_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appinfo: Option<KkmhAppinfo>,
    pub monitor: KkmhMonitor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universal_link: Option<String>,
    pub interact_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_app_path: Option<String>,
}
