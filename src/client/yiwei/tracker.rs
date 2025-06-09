use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct YiweiTracker {
    pub event_type: String,
    pub urls: Vec<String>,
}
