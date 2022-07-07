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
pub struct CreateServer {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
}

impl CreateServer {
    pub fn new(kind: String, name: String) -> CreateServer {
        CreateServer {
            kind,
            name,
            properties: None,
        }
    }
}

