use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SweetCaidList {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
}
