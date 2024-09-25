use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MfocusRequest {
    pub id: String,
    pub vdid: String,
    pub dealid: String,
    pub os: i32,
    pub ip: String,
    pub av: String,
    pub idfa: String,
    pub idfa_md5: String,
    pub caid: String,
    pub caid_md5: String,
    pub imei: String,
    pub imei_md5: String,
    pub oaid: String,
    pub oaid_md5: String,
    pub androidid_md5: String,
    pub ua: String,
    pub mac: String,
    pub model: String,
    pub sc_w: String,
    pub sc_h: String,
    pub mid: String,
    pub imsi: String,
    pub network: String,
    pub brand: String,
}
