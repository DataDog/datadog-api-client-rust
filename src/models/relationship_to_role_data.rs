/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// RelationshipToRoleData : Relationship to role object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipToRoleData {
    /// The unique identifier of the role.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::RolesType>,
}

impl RelationshipToRoleData {
    /// Relationship to role object.
    pub fn new() -> RelationshipToRoleData {
        RelationshipToRoleData {
            id: None,
            r#type: None,
        }
    }
}


