use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobSize {
    pub w: i32,
    pub h: i32,
}
