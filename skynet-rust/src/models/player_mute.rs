/*
 * Skynet API
 *
 * This is the skynet infrastructure api
 *
 * The version of the OpenAPI document: 0.1
 * Contact: contact@guillaume-etheve.fr
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerMute {
    /// Duration in seconds
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "unban", skip_serializing_if = "Option::is_none")]
    pub unban: Option<bool>,
}

impl PlayerMute {
    pub fn new() -> PlayerMute {
        PlayerMute {
            duration: None,
            reason: None,
            issuer: None,
            unban: None,
        }
    }
}

