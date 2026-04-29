use serde::{Deserialize, Serialize};

use super::{TianzhuoApp, TianzhuoContent, TianzhuoDevice, TianzhuoImp, TianzhuoSite, TianzhuoUser};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoRequest {
    pub id: String,
    pub imp: Vec<TianzhuoImp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<TianzhuoApp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<TianzhuoSite>,
    pub device: TianzhuoDevice,
    pub media_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wseat: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<TianzhuoUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TianzhuoContent>,
    pub test: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<i32>,
    pub is_https: bool,
}
