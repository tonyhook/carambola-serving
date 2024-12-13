use serde::{Deserialize, Deserializer, Serialize};
use derivative::Derivative;

use super::{FwbDataFormat, FwbImgFormat, FwbNativeVideoFormat, FwbTitleFormat};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(Derivative)]
#[derivative(Default)]
pub struct FwbAssetFormat {
    pub id: i32,
    #[serde(default)]
    #[derivative(Default(value = "0"))]
    #[serde(deserialize_with = "deserialize_null_default")]
    pub isrequired: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<FwbTitleFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<FwbImgFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<FwbNativeVideoFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FwbDataFormat>,
}

pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
