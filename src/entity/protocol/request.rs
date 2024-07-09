use serde::{Deserialize, Serialize};

use super::{Item, Context};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Request {
    pub id: String,
    pub item: Vec<Item>,
    pub context: Context,
}
