/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// RelationshipToOrganizations : Relationship to organizations.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipToOrganizations {
    /// Relationships to organization objects.
    #[serde(rename = "data")]
    pub data: Vec<crate::models::RelationshipToOrganizationData>,
}

impl RelationshipToOrganizations {
    /// Relationship to organizations.
    pub fn new(data: Vec<crate::models::RelationshipToOrganizationData>) -> RelationshipToOrganizations {
        RelationshipToOrganizations {
            data,
        }
    }
}


