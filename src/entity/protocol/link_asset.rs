use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct LinkAsset {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub linktype: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universallink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storeid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quickapplink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechatmppath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechatmpid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloadurl: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlfb: Option<String>,
}
