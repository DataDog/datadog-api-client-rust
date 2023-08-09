/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// RelationshipToUserData : Relationship to user object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipToUserData {
    /// A unique identifier that represents the user.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::UsersType,
}

impl RelationshipToUserData {
    /// Relationship to user object.
    pub fn new(id: String, r#type: crate::models::UsersType) -> RelationshipToUserData {
        RelationshipToUserData {
            id,
            r#type,
        }
    }
}


