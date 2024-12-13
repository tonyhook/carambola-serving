use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct FwbVideoFormat {
    pub w:i32,
    pub h: i32,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub videotype: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minduration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxduration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdelay: Option<i32>,
    pub mime: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<i32>,
}
