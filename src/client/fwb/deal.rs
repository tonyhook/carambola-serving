use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct FwbDeal {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<f64>,
}
