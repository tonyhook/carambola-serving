use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanAdGpsInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amap_code: Option<String>,
}
