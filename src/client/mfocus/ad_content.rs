use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MfocusAdContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mainimage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverimage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videourl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
