use serde::{Deserialize, Serialize};

use super::KkmhVideo;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KkmhAdm {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    pub creative_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<KkmhVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}
