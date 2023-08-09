/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentServiceRelationships : The incident service's relationships.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentServiceRelationships {
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::RelationshipToUser>>,
    #[serde(rename = "last_modified_by", skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<Box<crate::models::RelationshipToUser>>,
}

impl IncidentServiceRelationships {
    /// The incident service's relationships.
    pub fn new() -> IncidentServiceRelationships {
        IncidentServiceRelationships {
            created_by: None,
            last_modified_by: None,
        }
    }
}


