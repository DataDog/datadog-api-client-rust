/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentServiceIncludedItems : An object related to an incident service which is present in the included payload.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentServiceIncludedItems {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::UserAttributes>>,
    /// ID of the user.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Box<crate::models::UserResponseRelationships>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::UsersType>,
}

impl IncidentServiceIncludedItems {
    /// An object related to an incident service which is present in the included payload.
    pub fn new() -> IncidentServiceIncludedItems {
        IncidentServiceIncludedItems {
            attributes: None,
            id: None,
            relationships: None,
            r#type: None,
        }
    }
}


