use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkMiniProgram {
    #[serde(rename(deserialize = "mpUserName", serialize = "mpUserName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_user_name: Option<String>,
    #[serde(rename(deserialize = "mpPath", serialize = "mpPath"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_path: Option<String>,
}
