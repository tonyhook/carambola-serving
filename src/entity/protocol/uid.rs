use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Uid {
    pub atype: i32,
    pub id: String,
    pub ver: Option<String>,
    pub vendor: Option<String>,
    pub time: Option<i64>,
}
