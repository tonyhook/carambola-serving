use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct KkmhCaid {
    pub caid: String,
    pub caid_version: String,
}
