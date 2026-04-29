use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoLogo {
    pub w: Option<i32>,
    pub h: Option<i32>,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub logo_type: Option<i32>,
    pub url: Option<String>,
}
