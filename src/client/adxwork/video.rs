use serde::{Deserialize, Serialize};

use super::AdxworkTrack;

#[derive(Serialize, Deserialize)]
pub struct AdxworkVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endcard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endhtml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtype: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dur: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keepdur: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impurls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plays: Option<Vec<AdxworkTrack>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clkurls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpkg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}
