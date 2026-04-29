use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoMacros {
    #[serde(rename(deserialize = "macro", serialize = "macro"))]
    pub macro_name: Option<String>,
    pub value: Option<String>,
    pub expression: Option<Vec<String>>,
}
