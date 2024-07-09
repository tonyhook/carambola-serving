use serde::{Deserialize, Serialize};

use super::DisplayPlacement;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Companion {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcm: Option<i32>,
    pub display: DisplayPlacement,
}
