use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="1")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="2")]
    pub gender: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="3")]
    pub age: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="4")]
    pub keywords: Option<String>,
}
