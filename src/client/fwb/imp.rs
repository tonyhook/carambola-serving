use serde::{Deserialize, Serialize};

use super::{FwbNativeFormat, FwbVideoFormat, FwbPmp};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct FwbImp {
    pub id: String,
    pub tagid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<FwbNativeFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<FwbVideoFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isdeeplink: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isdownload: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isul: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmp: Option<FwbPmp>,
}
