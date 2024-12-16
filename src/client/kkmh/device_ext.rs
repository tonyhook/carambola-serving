use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KkmhDeviceExt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_id_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<i32>,
}
