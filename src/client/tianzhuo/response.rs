use serde::{Deserialize, Serialize};

use super::{TianzhuoExt, TianzhuoSeat};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoResponse {
    pub code: i32,
    pub msg: Option<String>,
    pub data: Option<TianzhuoResponseData>,
}

#[derive(Serialize, Deserialize)]
pub struct TianzhuoResponseData {
    pub id: Option<String>,
    pub bidid: Option<String>,
    pub seatbid: Option<Vec<TianzhuoSeat>>,
    pub cur: Option<String>,
    pub nbr: Option<i32>,
    pub debug_info: Option<String>,
    pub process_time_ms: Option<i64>,
    pub ext: Option<TianzhuoExt>,
}
