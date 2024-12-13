use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct FwbImgFormat {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub imagetype: i32,
    pub wmin: i32,
    pub hmin: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<String>>,
}
