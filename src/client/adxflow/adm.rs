use serde::{Deserialize, Serialize};

use super::{AdxflowExt, AdxflowImg, AdxflowRespApp, AdxflowVideo, AdxflowWx};

#[derive(Serialize, Deserialize)]
pub struct AdxflowAdm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<Vec<AdxflowImg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<AdxflowVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub land: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AdxflowRespApp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wx: Option<AdxflowWx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<AdxflowExt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crid: Option<String>,
}
