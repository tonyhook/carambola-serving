use serde::{Deserialize, Deserializer, Serialize};
use derivative::Derivative;

use super::Companion;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(Derivative)]
#[derivative(Default)]
pub struct VideoAssetFormat {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videotype: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[serde(default)]
    #[derivative(Default(value = "0"))]
    #[serde(deserialize_with = "deserialize_null_default")]
    pub skipmin: i32,
    #[serde(default)]
    #[derivative(Default(value = "0"))]
    #[serde(deserialize_with = "deserialize_null_default")]
    pub skipafter: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mindur: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxdur: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reqdurs: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minbitr: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxbitr: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxsize: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comp: Option<Vec<Companion>>,
}

pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
