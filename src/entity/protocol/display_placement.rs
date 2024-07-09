use serde::{Deserialize, Deserializer, Serialize};
use derivative::Derivative;

use super::{DisplayFormat, NativeFormat};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(Derivative)]
#[derivative(Default)]
pub struct DisplayPlacement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,
    #[serde(default)]
    #[derivative(Default(value = "0"))]
    #[serde(deserialize_with = "deserialize_null_default")]
    pub instl: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(default)]
    #[derivative(Default(value = "1"))]
    #[serde(deserialize_with = "deserialize_null_default")]
    pub unit: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayfmt: Option<DisplayFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nativefmt: Option<NativeFormat>,
}

pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
