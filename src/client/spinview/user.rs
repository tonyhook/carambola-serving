use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SpinviewUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yob: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jd: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taobao: Option<bool>,
}
