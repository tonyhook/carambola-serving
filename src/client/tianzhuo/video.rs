use serde::{Deserialize, Serialize};

use super::TianzhuoImage;

#[derive(Serialize, Deserialize)]
pub struct TianzhuoVideo {
    pub url: Option<String>,
    pub h: Option<i32>,
    pub w: Option<i32>,
    pub duration: Option<i32>,
    pub conver_image: Option<TianzhuoImage>,
}
