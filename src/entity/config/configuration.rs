use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone, Copy)]
pub struct Configuration {
    pub id: i32,
    pub log_transaction: bool,
    pub limit_request_frequency: i32,
    pub af_ip_frequency_hourly: i32,
    pub af_ip_frequency_daily: i32,
    pub af_id_frequency_hourly: i32,
    pub af_id_frequency_daily: i32,
    pub af_ua_per_id_hourly: i32,
    pub af_ua_per_id_daily: i32,
    pub af_ip_per_id_hourly: i32,
    pub af_ip_per_id_daily: i32,
}
