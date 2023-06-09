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
pub struct MessageComponent {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "font", skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(rename = "modifiers", skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Box<crate::models::MessageComponentModifiers>>,
}

impl MessageComponent {
    pub fn new(text: String) -> MessageComponent {
        MessageComponent {
            text,
            color: None,
            font: None,
            modifiers: None,
        }
    }
}


