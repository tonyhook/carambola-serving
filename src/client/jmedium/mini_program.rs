use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumMiniProgram {
    #[serde(rename(deserialize = "miniProgramId", serialize = "miniProgramId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_program_id: Option<String>,
    #[serde(rename(deserialize = "miniProgramPath", serialize = "miniProgramPath"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_program_path: Option<String>,
}
