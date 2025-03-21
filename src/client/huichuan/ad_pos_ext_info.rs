use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanAdPosExtInfo {
    pub key: String,
    pub value: String,
}
