use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumExt {
    #[serde(rename(deserialize = "supportWechat", serialize = "supportWechat"))]
    pub support_wechat: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<String>,
}
