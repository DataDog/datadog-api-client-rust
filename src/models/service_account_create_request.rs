/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ServiceAccountCreateRequest : Create a service account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceAccountCreateRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::ServiceAccountCreateData>,
}

impl ServiceAccountCreateRequest {
    /// Create a service account.
    pub fn new(data: crate::models::ServiceAccountCreateData) -> ServiceAccountCreateRequest {
        ServiceAccountCreateRequest {
            data: Box::new(data),
        }
    }
}


