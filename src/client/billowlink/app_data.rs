use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkAppData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,
     #[serde(skip_serializing_if = "Option::is_none")]
   pub ver: Option<String>,
    #[serde(rename(deserialize = "storeUrl", serialize = "storeUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer: Option<String>,
    #[serde(rename(deserialize = "privacyUrl", serialize = "privacyUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(rename(deserialize = "paymentType", serialize = "paymentType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
}
