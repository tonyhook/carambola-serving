use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ZhanqingCaid {
    pub id: String,
    pub ver: String,
}
