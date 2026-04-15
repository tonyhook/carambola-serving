use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdxworkApp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adpkg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
}
