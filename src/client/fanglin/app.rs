use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FanglinApp {
    pub bundle: String,
    pub app_ver: String,
    pub app_name: String,
}
