use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Options {
    String(String),
    Category(Vec<String>),
    Number(f64),
    Boolean(bool),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Rule {
    pub field: String,
    pub operator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Options>,
}
