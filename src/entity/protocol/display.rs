use serde::{Deserialize, Serialize};

use super::{Banner, Event, Native};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Display {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<Banner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<Native>,
    pub event: Vec<Event>,
}
