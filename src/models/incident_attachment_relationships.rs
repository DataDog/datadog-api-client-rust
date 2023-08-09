/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentAttachmentRelationships : The incident attachment's relationships.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentAttachmentRelationships {
    #[serde(rename = "last_modified_by_user", skip_serializing_if = "Option::is_none")]
    pub last_modified_by_user: Option<Box<crate::models::RelationshipToUser>>,
}

impl IncidentAttachmentRelationships {
    /// The incident attachment's relationships.
    pub fn new() -> IncidentAttachmentRelationships {
        IncidentAttachmentRelationships {
            last_modified_by_user: None,
        }
    }
}


