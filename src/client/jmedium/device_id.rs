use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumDeviceId {
    pub imei: String,
    #[serde(rename(deserialize = "imeiMd5", serialize = "imeiMd5"))]
    pub imei_md5: String,
    pub oaid: String,
    #[serde(rename(deserialize = "oaidMd5", serialize = "oaidMd5"))]
    pub oaid_md5: String,
    #[serde(rename(deserialize = "androidId", serialize = "androidId"))]
    pub android_id: String,
    #[serde(rename(deserialize = "androidIdMd5", serialize = "androidIdMd5"))]
    pub android_id_md5: String,
    #[serde(rename(deserialize = "androidIdSha1", serialize = "androidIdSha1"))]
    pub android_id_sha1: String,
    pub idfa: String,
    #[serde(rename(deserialize = "idfaMd5", serialize = "idfaMd5"))]
    pub idfa_md5: String,
    pub idfv: String,
    #[serde(rename(deserialize = "openUdid", serialize = "openUdid"))]
    pub open_udid: String,
}
