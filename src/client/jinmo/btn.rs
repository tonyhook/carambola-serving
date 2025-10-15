use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoBtn {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag = "1")]
    pub btn_name: Option<String>,
}
