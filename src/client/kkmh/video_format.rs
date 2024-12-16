use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct KkmhVideoFormat {
    pub mimes: Vec<String>,
}
