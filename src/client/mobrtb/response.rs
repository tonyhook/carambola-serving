use serde::{Deserialize, Serialize};

use super::MobrtbAd;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MobrtbResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub ads: Vec<MobrtbAd>,
}
