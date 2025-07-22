use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongGeo {
    pub lat: f64,
    pub lon: f64,
    pub source: i32,
}
