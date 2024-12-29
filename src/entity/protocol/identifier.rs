use serde::{Deserialize, Serialize};

use super::Uid;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Identifier {
    pub uids: Vec<Uid>,
}
