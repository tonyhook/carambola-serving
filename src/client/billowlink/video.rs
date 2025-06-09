use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverw: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverh: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(rename(deserialize = "afterHtml", serialize = "afterHtml"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_html: Option<String>,
}
