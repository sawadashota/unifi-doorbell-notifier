use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapResponse {
    pub cameras: Vec<Camera>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    pub mac: String,
    pub host: String,
    pub connection_host: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub state: String,
    pub is_managed: bool,
    pub last_ring: Option<i64>,
    pub id: String,
}

impl Camera {
    pub fn is_doorbell(&self) -> bool {
        self.is_managed && self.type_field == "UVC G4 Doorbell"
    }
}
