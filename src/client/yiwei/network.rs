use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct YiweiNetwork {
    pub carrier: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnc: Option<i32>,
    pub conn_type: i32,
}
