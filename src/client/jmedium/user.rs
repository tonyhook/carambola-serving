use serde::{Deserialize, Serialize};

use super::JmediumInstalledApp;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest: Option<Vec<String>>,
    #[serde(rename(deserialize = "installedApps", serialize = "installedApps"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_apps: Option<Vec<JmediumInstalledApp>>,
}
