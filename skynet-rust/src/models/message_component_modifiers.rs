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
pub struct MessageComponentModifiers {
    #[serde(rename = "bold", skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(rename = "italic", skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(rename = "underlined", skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    #[serde(rename = "strikethrough", skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(rename = "obfuscated", skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
}

impl MessageComponentModifiers {
    pub fn new() -> MessageComponentModifiers {
        MessageComponentModifiers {
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
        }
    }
}

