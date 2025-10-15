use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoGeo {
    #[serde(rename = "lat")]
    #[prost(double, tag="1")]
    pub latitude: f64,
    #[serde(rename = "lng")]
    #[prost(double, tag="2")]
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="3")]
    pub coordinate_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="4")]
    pub laccu: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(double, optional, tag="5")]
    pub accuracy_m: Option<f64>,
}
