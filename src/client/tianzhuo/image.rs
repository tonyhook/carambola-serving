use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TianzhuoImage {
    pub id: Option<String>,
    pub url: Option<String>,
    pub h: Option<i32>,
    pub w: Option<i32>,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub image_type: Option<i32>,
}
