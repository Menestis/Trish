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
pub struct PlayerInfo {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "power")]
    pub power: i32,
    #[serde(rename = "locale")]
    pub locale: String,
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<i32>,
    #[serde(rename = "premium_currency", skip_serializing_if = "Option::is_none")]
    pub premium_currency: Option<i32>,
    #[serde(rename = "proxy", skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(rename = "blocked")]
    pub blocked: Vec<String>,
    #[serde(rename = "ban", skip_serializing_if = "Option::is_none")]
    pub ban: Option<Box<crate::models::Ban>>,
    #[serde(rename = "inventory")]
    pub inventory: ::std::collections::HashMap<String, i32>,
    #[serde(rename = "properties")]
    pub properties: ::std::collections::HashMap<String, String>,
    #[serde(rename = "discord_id", skip_serializing_if = "Option::is_none")]
    pub discord_id: Option<String>,
    #[serde(rename = "mute", skip_serializing_if = "Option::is_none")]
    pub mute: Option<Box<crate::models::Mute>>,
}

impl PlayerInfo {
    pub fn new(uuid: String, username: String, power: i32, locale: String, blocked: Vec<String>, inventory: ::std::collections::HashMap<String, i32>, properties: ::std::collections::HashMap<String, String>) -> PlayerInfo {
        PlayerInfo {
            uuid,
            username,
            power,
            locale,
            prefix: None,
            suffix: None,
            currency: None,
            premium_currency: None,
            proxy: None,
            server: None,
            blocked,
            ban: None,
            inventory,
            properties,
            discord_id: None,
            mute: None,
        }
    }
}

