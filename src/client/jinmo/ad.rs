use serde::{Deserialize, Serialize};

use super::{JinmoAdApp, JinmoInteraction, JinmoMaterial, JinmoLogo, JinmoBtn, JinmoTrack};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoAd {
    #[prost(string, tag="1")]
    pub imp_id: String,
    #[prost(string, tag="2")]
    pub track_id: String,
    #[prost(string, tag="3")]
    pub space_id: String,
    #[prost(int32, tag="4")]
    pub bid_type: i32,
    #[prost(int64, tag="5")]
    pub bid_price: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="6")]
    pub ad_app: Option<JinmoAdApp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="7")]
    pub interaction: Option<JinmoInteraction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="8")]
    pub material: Option<JinmoMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag="9")]
    pub expire_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="10")]
    pub incentive_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="11")]
    pub logo_info: Option<JinmoLogo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="12")]
    pub btn_info: Option<JinmoBtn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="13")]
    pub track_info: Option<JinmoTrack>,
}
