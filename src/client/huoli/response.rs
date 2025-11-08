use serde::{Deserialize, Serialize};

use super::HuoliSeat;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuoliResponse {
    pub id: String,
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errmsg: Option<String>,
    pub time: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat: Option<Vec<HuoliSeat>>,
}
