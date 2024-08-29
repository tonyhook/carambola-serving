use serde::{Deserialize, Deserializer, Serialize};
use derivative::Derivative;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Derivative)]
#[derivative(Default)]
pub struct AdwanjiApp {
    pub name: String,
    pub bundle: String,
    pub ver: String,
    #[serde(default)]
    #[derivative(Default(value = "0"))]
    #[serde(deserialize_with = "deserialize_null_default")]
    pub paid: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storeurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itunesid: Option<String>,
}

pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
