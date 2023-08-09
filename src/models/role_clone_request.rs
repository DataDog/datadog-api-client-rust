/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// RoleCloneRequest : Request to create a role by cloning an existing role.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RoleCloneRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::RoleClone>,
}

impl RoleCloneRequest {
    /// Request to create a role by cloning an existing role.
    pub fn new(data: crate::models::RoleClone) -> RoleCloneRequest {
        RoleCloneRequest {
            data: Box::new(data),
        }
    }
}


