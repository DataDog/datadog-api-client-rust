/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ApplicationKeyUpdateData : Object used to update an application key.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplicationKeyUpdateData {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::ApplicationKeyUpdateAttributes>,
    /// ID of the application key.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::ApplicationKeysType,
}

impl ApplicationKeyUpdateData {
    /// Object used to update an application key.
    pub fn new(attributes: crate::models::ApplicationKeyUpdateAttributes, id: String, r#type: crate::models::ApplicationKeysType) -> ApplicationKeyUpdateData {
        ApplicationKeyUpdateData {
            attributes: Box::new(attributes),
            id,
            r#type,
        }
    }
}


