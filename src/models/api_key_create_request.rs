/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ApiKeyCreateRequest : Request used to create an API key.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiKeyCreateRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::ApiKeyCreateData>,
}

impl ApiKeyCreateRequest {
    /// Request used to create an API key.
    pub fn new(data: crate::models::ApiKeyCreateData) -> ApiKeyCreateRequest {
        ApiKeyCreateRequest {
            data: Box::new(data),
        }
    }
}


