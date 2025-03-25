use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SweetApp {
    pub name: String,
    pub bundle: String,
    pub ver: String,
    #[serde(rename(deserialize = "storeUrl", serialize = "storeUrl"))]
    pub store_url: String,
}
