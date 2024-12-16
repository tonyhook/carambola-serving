use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct KkmhBannerFormat {
    pub w: i32,
    pub h: i32,
}
