use serde::{Deserialize, Serialize};

use super::LinkAsset;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Banner {
    pub img: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<LinkAsset>,
}
