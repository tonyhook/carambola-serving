use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkSite {
    #[serde(rename(deserialize = "siteID", serialize = "siteID"))]
    pub site_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(rename(deserialize = "ref", serialize = "ref"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<String>,
}
