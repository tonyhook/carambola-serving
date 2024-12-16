use serde::{Deserialize, Serialize};

use super::{KkmhBannerFormat, KkmhImpExt, KkmhNativeFormat, KkmhVideoFormat};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct KkmhImp {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ba: Option<KkmhBannerFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vd: Option<KkmhVideoFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub na: Option<KkmhNativeFormat>,
    pub tagid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_num: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<KkmhImpExt>,
}
