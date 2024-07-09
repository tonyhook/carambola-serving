use serde::{Deserialize, Serialize};

use super::Uid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Identifier {
    pub uids: Vec<Uid>,
}
