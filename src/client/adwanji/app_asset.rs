use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AdwanjiAppAsset {
    pub name: String,
    pub pack: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    #[serde(rename(deserialize = "iTunesID", serialize = "iTunesID"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itunesid: Option<String>,
}
