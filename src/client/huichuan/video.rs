use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanVideo {
    #[serde(rename(deserialize = "FD", serialize = "FD"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fd: Option<String>,
    #[serde(rename(deserialize = "LD", serialize = "LD"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ld: Option<String>,
}
