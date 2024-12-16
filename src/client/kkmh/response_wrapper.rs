use serde::{Deserialize, Serialize};

use super::KkmhResponse;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KkmhResponseWrapper {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<KkmhResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
