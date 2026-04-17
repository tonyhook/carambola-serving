use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct TengmeiUser {
    #[prost(string, repeated, tag="1")]
    pub install_apps: Vec<String>,
}
