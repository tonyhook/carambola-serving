use serde::{Deserialize, Serialize};

use super::AssetFormat;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct NativeFormat {
    pub asset: Vec<AssetFormat>,
}
