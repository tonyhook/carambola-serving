use serde::{Deserialize, Deserializer, Serialize};
use derivative::Derivative;

use super::DisplayPlacement;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Derivative)]
#[derivative(Default)]
pub struct Placement {
    pub tagid: String,
    #[serde(default)]
    #[derivative(Default(value = "0"))]
    #[serde(deserialize_with = "deserialize_null_default")]
    pub reward: i32,
    pub display: DisplayPlacement,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
