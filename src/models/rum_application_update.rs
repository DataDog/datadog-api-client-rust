/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// RumApplicationUpdate : RUM application update.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RumApplicationUpdate {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::RumApplicationUpdateAttributes>>,
    /// RUM application ID.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::RumApplicationUpdateType,
}

impl RumApplicationUpdate {
    /// RUM application update.
    pub fn new(id: String, r#type: crate::models::RumApplicationUpdateType) -> RumApplicationUpdate {
        RumApplicationUpdate {
            attributes: None,
            id,
            r#type,
        }
    }
}


