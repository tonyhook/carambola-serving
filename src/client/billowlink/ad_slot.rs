use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkAdSlot {
    #[serde(rename(deserialize = "slotID", serialize = "slotID"))]
    pub slot_id: String,
    #[serde(rename(deserialize = "adType", serialize = "adType"))]
    pub ad_type: i32,
    #[serde(rename(deserialize = "bidFloor", serialize = "bidFloor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_floor: Option<i64>,
    pub w: i32,
    pub h: i32,
    #[serde(rename(deserialize = "creativeType", serialize = "creativeType"))]
    pub creative_type: i32,
    #[serde(rename(deserialize = "minDuration", serialize = "minDuration"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration: Option<i32>,
    #[serde(rename(deserialize = "maxDuration", serialize = "maxDuration"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[serde(rename(deserialize = "skipAfter", serialize = "skipAfter"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_after: Option<i32>,
    #[serde(rename(deserialize = "videoType", serialize = "videoType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_type: Option<i32>,
    #[serde(rename(deserialize = "bannerType", serialize = "bannerType"))]
    pub banner_type: i32,
}
