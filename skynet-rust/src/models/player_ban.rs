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
pub struct PlayerBan {
    /// Duration in seconds
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<bool>,
    #[serde(rename = "unban", skip_serializing_if = "Option::is_none")]
    pub unban: Option<bool>,
}

impl PlayerBan {
    pub fn new() -> PlayerBan {
        PlayerBan {
            duration: None,
            reason: None,
            issuer: None,
            ip: None,
            unban: None,
        }
    }
}

