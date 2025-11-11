use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobText {
    pub title: String,
    pub desc: String,
}
