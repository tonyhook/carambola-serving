use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct TengmeiApp {
    #[prost(string, tag="1")]
    pub id: String,
    #[prost(string, tag="2")]
    pub name: String,
    #[prost(string, tag="3")]
    pub bundle: String,
    #[prost(string, tag="4")]
    pub version: String,
}
