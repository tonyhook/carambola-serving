use serde::{Deserialize, Serialize};

use super::HuoliAd;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct HuoliSeat {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad: Option<Vec<HuoliAd>>,
}
