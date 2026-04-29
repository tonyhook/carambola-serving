use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoDeal {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wseat: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wadomain: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloorcur: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<i32>,
}
