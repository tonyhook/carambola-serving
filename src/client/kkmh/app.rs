use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KkmhApp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub bundle: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}
