use serde::{Deserialize, Deserializer, Serialize};
use derivative::Derivative;

use super::{FwbDataAsset, FwbImgAsset, FwbNativeVideoAsset, FwbTitleAsset};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Derivative)]
#[derivative(Default)]
pub struct FwbAsset {
    pub id: i32,
    #[serde(default)]
    #[derivative(Default(value = "0"))]
    #[serde(deserialize_with = "deserialize_null_default")]
    pub isrequired: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<FwbTitleAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<FwbImgAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<FwbNativeVideoAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FwbDataAsset>,
}

pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
