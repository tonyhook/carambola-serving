use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KkmhVideo {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
}
