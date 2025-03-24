use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct RuiangCaid {
    pub caid: String,
    #[serde(rename(deserialize = "caidVersion", serialize = "caidVersion"))]
    pub caid_version: String,
}
