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
pub struct Mute {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "start")]
    pub start: String,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i32>,
}

impl Mute {
    pub fn new(id: String, start: String) -> Mute {
        Mute {
            id,
            start,
            end: None,
            issuer: None,
            reason: None,
            target: None,
            remaining: None,
        }
    }
}


