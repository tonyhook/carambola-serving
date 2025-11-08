use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuoliGeo {
    pub lat: f64,
    pub lon: f64,
}
