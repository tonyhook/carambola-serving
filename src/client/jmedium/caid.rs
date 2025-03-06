use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumCaid {
    pub caid: String,
    pub version: String,
    #[serde(rename(deserialize = "generateTime", serialize = "generateTime"))]
    pub generate_time: i64,
    pub vendor: i32,
}
