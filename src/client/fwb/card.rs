use serde::{Deserialize, Serialize};

use super::FwbButton;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FwbCard {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardtype: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<FwbButton>,
}
