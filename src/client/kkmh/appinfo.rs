use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KkmhAppinfo {
    pub name: String,
    pub bundle: String,
    pub version: String,
    pub icon: String,
    pub app_id: i64,
}
