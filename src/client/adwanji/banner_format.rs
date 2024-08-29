use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AdwanjiBannerFormat {
    pub w: i32,
    pub h: i32,
    pub pos: i32,
}
