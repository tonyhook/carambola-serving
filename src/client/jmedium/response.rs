use serde::{Deserialize, Serialize};

use super::JmediumAdInfo;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumResponse {
    pub code: i32,
    pub data: Option<Vec<JmediumAdInfo>>,
    pub msg: String,
}
