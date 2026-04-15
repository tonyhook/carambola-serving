use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OnenmobUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
}
