use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SweetImp {
    #[serde(rename(deserialize = "tagId", serialize = "tagId"))]
    pub tag_id: String,
    pub w: i32,
    pub h: i32,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub imptype: i32,
    pub pos: i32,
    #[serde(rename(deserialize = "cType", serialize = "cType"))]
    pub c_type: Vec<i32>,
    #[serde(rename(deserialize = "ciType", serialize = "ciType"))]
    pub ci_type: Vec<i32>,
    #[serde(rename(deserialize = "dp", serialize = "dp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dp: Option<i32>,
    #[serde(rename(deserialize = "bidFloor", serialize = "bidFloor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_floor: Option<i32>,
}
