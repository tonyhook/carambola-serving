use serde::{Deserialize, Serialize};

use super::LeidongAppResponse;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongInteraction {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub interaction_type: String,
    pub target_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<LeidongAppResponse>,
    #[serde(rename(deserialize = "originId", serialize = "originId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    #[serde(rename(deserialize = "deeplinkUrl", serialize = "deeplinkUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_url: Option<String>,
}
