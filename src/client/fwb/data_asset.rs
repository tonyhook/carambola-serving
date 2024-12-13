use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FwbDataAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub value: String,
}
