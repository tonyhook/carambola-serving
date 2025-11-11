use serde::{Deserialize, Serialize};

use super::MvpmobAppResponse;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobInteraction {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub interaction_type: String,
    pub target_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<MvpmobAppResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_url: Option<String>,
}
