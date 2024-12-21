use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MobrtbApp {
    pub name: String,
    pub version: String,
    pub bundle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_mode: Option<i32>,
}
