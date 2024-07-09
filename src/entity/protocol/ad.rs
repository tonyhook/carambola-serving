use serde::{Deserialize, Serialize};

use super::Display;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Ad {
    pub id: String,
    pub display: Display,
}
