use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FanglinUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<String>>,
}
