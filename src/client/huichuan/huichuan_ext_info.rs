use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanHuichuanExtInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_idea_ids: Option<String>,
}
