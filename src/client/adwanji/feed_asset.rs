use serde::{Deserialize, Serialize};

use super::AdwanjiBannerAsset;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AdwanjiFeedAsset {
    pub title: String,
    pub desc: String,
    pub imgs: Vec<AdwanjiBannerAsset>,
}
