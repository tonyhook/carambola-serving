use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumDeal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<i32>,
}
