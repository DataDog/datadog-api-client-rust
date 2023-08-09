/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// RelationshipToPermission : Relationship to a permissions object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipToPermission {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::RelationshipToPermissionData>>,
}

impl RelationshipToPermission {
    /// Relationship to a permissions object.
    pub fn new() -> RelationshipToPermission {
        RelationshipToPermission {
            data: None,
        }
    }
}


