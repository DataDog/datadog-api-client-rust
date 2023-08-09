/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UserCreateRequest : Create a user.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserCreateRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::UserCreateData>,
}

impl UserCreateRequest {
    /// Create a user.
    pub fn new(data: crate::models::UserCreateData) -> UserCreateRequest {
        UserCreateRequest {
            data: Box::new(data),
        }
    }
}


