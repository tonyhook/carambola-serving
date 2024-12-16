use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct KkmhNativeFormat {
    pub layout: i32,
    pub w: i32,
    pub h: i32,
}
