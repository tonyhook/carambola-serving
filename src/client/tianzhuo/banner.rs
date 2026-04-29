use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoBanner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btype: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmax: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmax: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmin: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmin: Option<i32>,
}
