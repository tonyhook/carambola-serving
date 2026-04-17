use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct TengmeiGeo {
    #[prost(double, tag="1")]
    pub lat: f64,
    #[prost(double, tag="2")]
    pub lon: f64,
}
