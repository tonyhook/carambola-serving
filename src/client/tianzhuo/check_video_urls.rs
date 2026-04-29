use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoCheckVideoUrls {
    pub url: Option<Vec<String>>,
    pub time: Option<f64>,
}
