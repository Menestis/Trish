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
pub struct OnlinePlayerInfo {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "session")]
    pub session: String,
    #[serde(rename = "proxy")]
    pub proxy: String,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

impl OnlinePlayerInfo {
    pub fn new(uuid: String, username: String, session: String, proxy: String) -> OnlinePlayerInfo {
        OnlinePlayerInfo {
            uuid,
            username,
            session,
            proxy,
            server: None,
        }
    }
}

