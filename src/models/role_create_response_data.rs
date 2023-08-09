/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// RoleCreateResponseData : Role object returned by the API.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RoleCreateResponseData {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::RoleCreateAttributes>>,
    /// The unique identifier of the role.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Box<crate::models::RoleResponseRelationships>>,
    #[serde(rename = "type")]
    pub r#type: crate::models::RolesType,
}

impl RoleCreateResponseData {
    /// Role object returned by the API.
    pub fn new(r#type: crate::models::RolesType) -> RoleCreateResponseData {
        RoleCreateResponseData {
            attributes: None,
            id: None,
            relationships: None,
            r#type,
        }
    }
}


