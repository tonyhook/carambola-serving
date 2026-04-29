use serde::{Deserialize, Serialize};

use super::{TianzhuoBanner, TianzhuoNativeRequest, TianzhuoPmp, TianzhuoVideoRequest};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoImp {
    pub id: String,
    pub tagid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowstyle: Option<Vec<String>>,
    pub ad_slot_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<TianzhuoBanner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<TianzhuoVideoRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<TianzhuoNativeRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloorcur: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmp: Option<TianzhuoPmp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_num: Option<i32>,
}
