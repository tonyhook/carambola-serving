use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanExpTags {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<i32>,
}
