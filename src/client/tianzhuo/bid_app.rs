use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoBidApp {
    pub app_name: Option<String>,
    pub bundle: Option<String>,
    pub app_icon: Option<String>,
    pub app_size: Option<i64>,
}
