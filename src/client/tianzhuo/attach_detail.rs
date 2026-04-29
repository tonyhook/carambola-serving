use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoAttachDetail {
    pub button_type: Option<String>,
    pub button_text: Option<String>,
    pub attach_url: Option<String>,
    pub phone_number: Option<String>,
}
