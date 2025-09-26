use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KakaNetwork {
    #[serde(rename = "connectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "operatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_type: Option<String>,
}
