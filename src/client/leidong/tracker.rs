use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongTracker {
    pub event_type: String,
    pub tracker_urls: Vec<String>,
}
