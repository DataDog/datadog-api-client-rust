/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentIntegrationMetadataResponseData : Incident integration metadata from a response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentIntegrationMetadataResponseData {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::IncidentIntegrationMetadataAttributes>>,
    /// The incident integration metadata's ID.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::IncidentIntegrationMetadataType,
}

impl IncidentIntegrationMetadataResponseData {
    /// Incident integration metadata from a response.
    pub fn new(id: String, r#type: crate::models::IncidentIntegrationMetadataType) -> IncidentIntegrationMetadataResponseData {
        IncidentIntegrationMetadataResponseData {
            attributes: None,
            id,
            r#type,
        }
    }
}


