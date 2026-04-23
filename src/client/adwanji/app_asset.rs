use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AdwanjiAppAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename(deserialize = "permissionsUrl", serialize = "permissionsUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_url: Option<String>,
    #[serde(rename(deserialize = "privacyAgreement", serialize = "privacyAgreement"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_agreement: Option<String>,
    #[serde(rename(deserialize = "descriptionUrl", serialize = "descriptionUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    #[serde(rename(deserialize = "iTunesID", serialize = "iTunesID"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itunesid: Option<String>,
}
