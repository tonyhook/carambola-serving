use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ZhanqingApp {
    pub appver: String,
    pub appname: String,
    pub pkgname: String,
    pub w: i32,
    pub h: i32,
}
