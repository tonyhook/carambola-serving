use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumApp {
    pub name: String,
    #[serde(rename(deserialize = "verName", serialize = "verName"))]
    pub ver_name: String,
    #[serde(rename(deserialize = "verCode", serialize = "verCode"))]
    pub ver_code: i32,
    #[serde(rename(deserialize = "pkgName", serialize = "pkgName"))]
    pub pkg_name: String,
    #[serde(rename(deserialize = "appStoreVersion", serialize = "appStoreVersion"))]
    pub app_store_version: String,
}
