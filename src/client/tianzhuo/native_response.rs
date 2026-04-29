use serde::{Deserialize, Serialize};

use super::{TianzhuoIcon, TianzhuoImage, TianzhuoLogo, TianzhuoVideo};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoNativeResponse {
    pub ver: Option<String>,
    pub icon: Option<TianzhuoIcon>,
    pub logo: Option<TianzhuoLogo>,
    pub images: Option<Vec<TianzhuoImage>>,
    pub video: Option<TianzhuoVideo>,
    pub title: Option<String>,
    pub desc: Option<String>,
    pub desc2: Option<String>,
}
