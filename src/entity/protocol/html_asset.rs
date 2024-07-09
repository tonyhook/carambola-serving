use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct HtmlAsset {
    pub html: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,
}
