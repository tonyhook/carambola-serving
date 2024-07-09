use serde::{Deserialize, Serialize};

use super::{Asset, LinkAsset};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Native {
    pub asset: Vec<Asset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<LinkAsset>,
}
