use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongIcon {
    pub height: i32,
    pub width: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
