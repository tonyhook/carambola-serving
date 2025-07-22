use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongImp {
    #[serde(rename(deserialize = "floorPrice", serialize = "floorPrice"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
}
