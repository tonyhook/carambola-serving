use serde::{Deserialize, Serialize};

use super::FwbAssetFormat;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct FwbNativeFormat {
    pub assets: Vec<FwbAssetFormat>,
    pub layout: i32,
}
