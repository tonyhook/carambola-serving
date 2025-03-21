use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanAdExtInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_forbidden: Option<i32>,
}
