use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct YiweiPermission {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub desc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk: Option<String>,
}
