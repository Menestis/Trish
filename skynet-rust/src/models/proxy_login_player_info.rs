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
pub struct ProxyLoginPlayerInfo {
    #[serde(rename = "power")]
    pub power: i32,
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
    #[serde(rename = "locale")]
    pub locale: String,
    #[serde(rename = "properties")]
    pub properties: ::std::collections::HashMap<String, String>,
}

impl ProxyLoginPlayerInfo {
    pub fn new(power: i32, permissions: Vec<String>, locale: String, properties: ::std::collections::HashMap<String, String>) -> ProxyLoginPlayerInfo {
        ProxyLoginPlayerInfo {
            power,
            permissions,
            locale,
            properties,
        }
    }
}


