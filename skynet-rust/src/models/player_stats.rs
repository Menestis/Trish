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
pub struct PlayerStats {
    #[serde(rename = "session")]
    pub session: String,
    #[serde(rename = "server")]
    pub server: String,
    #[serde(rename = "stats")]
    pub stats: ::std::collections::HashMap<String, i32>,
}

impl PlayerStats {
    pub fn new(session: String, server: String, stats: ::std::collections::HashMap<String, i32>) -> PlayerStats {
        PlayerStats {
            session,
            server,
            stats,
        }
    }
}


