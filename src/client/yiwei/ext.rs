use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct YiweiExt {
    pub key: String,
    pub value: String,
}
