use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MobrtbVideo {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}
