use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OnenmobCaid {
    pub id: String,
    pub version: String,
}
