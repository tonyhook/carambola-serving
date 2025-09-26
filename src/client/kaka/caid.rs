use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KakaCaid {
    pub version: String,
    pub caid: String,
}
