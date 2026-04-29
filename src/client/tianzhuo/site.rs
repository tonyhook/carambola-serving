use serde::{Deserialize, Serialize};

use super::{TianzhuoContent, TianzhuoPublisher};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoSite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(rename(deserialize = "ref", serialize = "ref"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<TianzhuoPublisher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TianzhuoContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
}
