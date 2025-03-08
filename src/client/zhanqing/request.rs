use serde::{Deserialize, Serialize};

use super::{ZhanqingApp, ZhanqingDevice};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ZhanqingRequest {
    pub rid: String,
    pub pid: String,
    pub bidfloor: i32,
    pub time: u128,
    pub token: String,
    pub appinfo: ZhanqingApp,
    pub deviceinfo: ZhanqingDevice,
}
