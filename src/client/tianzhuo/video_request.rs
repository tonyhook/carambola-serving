use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoVideoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minduration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxduration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdelay: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minbitrate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxbitrate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linearity: Option<i32>,
}
