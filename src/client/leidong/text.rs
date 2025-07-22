use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongText {
    pub title: String,
    pub desc: String,
}
