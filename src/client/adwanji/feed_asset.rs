use serde::{Deserialize, Serialize};

use super::AdwanjiBannerAsset;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AdwanjiFeedAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgs: Option<Vec<AdwanjiBannerAsset>>,
}
