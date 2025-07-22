use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongSize {
    pub w: i32,
    pub h: i32,
}
