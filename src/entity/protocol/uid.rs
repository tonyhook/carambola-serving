use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Uid {
    pub atype: i32,
    pub id: String,
}
