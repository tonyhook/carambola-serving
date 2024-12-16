use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct KkmhImpExt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_types: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_types: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
}
