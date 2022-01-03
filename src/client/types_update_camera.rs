use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCameraRequest {
    pub lcd_message: LcdMessage,
}

impl UpdateCameraRequest {
    pub fn update_message(message: impl Into<String>, duration: Duration) -> Self {
        let mut req = Self::default();
        req.lcd_message = LcdMessage {
            type_field: "CUSTOM_MESSAGE".to_string(),
            text: message.into(),
            duration: duration.num_seconds(),
        };
        req
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
    pub duration: i64,
}
