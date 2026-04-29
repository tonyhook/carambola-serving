use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoAdvertiser {
    pub id: Option<String>,
    pub industry: Option<String>,
    pub sub_industry: Option<String>,
}
