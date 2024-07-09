use serde::{Deserialize, Deserializer, Serialize};
use derivative::Derivative;

use super::{AppAsset, DataAsset, HtmlAsset, ImageAsset, TitleAsset, VideoAsset};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(Derivative)]
#[derivative(Default)]
pub struct Asset {
    pub id: i32,
    #[serde(default)]
    #[derivative(Default(value = "0"))]
    #[serde(deserialize_with = "deserialize_null_default")]
    pub req: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<TitleAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<ImageAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DataAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<HtmlAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppAsset>,
}

pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
