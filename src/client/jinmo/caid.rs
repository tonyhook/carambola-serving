use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoCaid {
    #[prost(string, tag="1")]
    pub version: String,
    #[prost(string, tag="2")]
    pub caid: String,
}
