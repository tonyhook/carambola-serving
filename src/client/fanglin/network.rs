use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FanglinNetwork {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    pub conn_type: i32,
    pub operator_type: i32,
}
