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
pub struct PlayerTransaction {
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<i32>,
    #[serde(rename = "premium_currency", skip_serializing_if = "Option::is_none")]
    pub premium_currency: Option<i32>,
}

impl PlayerTransaction {
    pub fn new() -> PlayerTransaction {
        PlayerTransaction {
            currency: None,
            premium_currency: None,
        }
    }
}


