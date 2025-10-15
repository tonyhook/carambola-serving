use serde::{Deserialize, Serialize};

use super::JinmoImage;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoLogo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="1")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="2")]
    pub image: Option<JinmoImage>,
}
