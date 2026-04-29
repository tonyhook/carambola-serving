use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct TianzhuoExt {
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}
