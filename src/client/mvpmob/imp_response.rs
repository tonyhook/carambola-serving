use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobImpResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
}
