use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FanglinNetwork {
    pub ip: String,
    pub conn_type: i32,
    pub operator_type: i32,
}
