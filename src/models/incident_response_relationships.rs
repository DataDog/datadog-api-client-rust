/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentResponseRelationships : The incident's relationships from a response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentResponseRelationships {
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Box<crate::models::RelationshipToIncidentAttachment>>,
    #[serde(rename = "commander_user", skip_serializing_if = "Option::is_none")]
    pub commander_user: Option<Box<crate::models::NullableRelationshipToUser>>,
    #[serde(rename = "created_by_user", skip_serializing_if = "Option::is_none")]
    pub created_by_user: Option<Box<crate::models::RelationshipToUser>>,
    #[serde(rename = "integrations", skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Box<crate::models::RelationshipToIncidentIntegrationMetadatas>>,
    #[serde(rename = "last_modified_by_user", skip_serializing_if = "Option::is_none")]
    pub last_modified_by_user: Option<Box<crate::models::RelationshipToUser>>,
}

impl IncidentResponseRelationships {
    /// The incident's relationships from a response.
    pub fn new() -> IncidentResponseRelationships {
        IncidentResponseRelationships {
            attachments: None,
            commander_user: None,
            created_by_user: None,
            integrations: None,
            last_modified_by_user: None,
        }
    }
}


