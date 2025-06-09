use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct YiweiReward {
    pub reward_item: String,
    pub amount: i32,
    pub condition_type: i32,
    pub orientation: i32,
}
