use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumGeo {
    #[serde(rename(deserialize = "coordinateType", serialize = "coordinateType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
}
