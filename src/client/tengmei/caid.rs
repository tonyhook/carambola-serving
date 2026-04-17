use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct TengmeiCaid {
    #[prost(string, tag="1")]
    pub caid: String,
    #[prost(string, tag="2")]
    pub version: String,
}
