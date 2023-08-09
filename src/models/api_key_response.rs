/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ApiKeyResponse : Response for retrieving an API key.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::FullApiKey>>,
    /// Array of objects related to the API key.
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<crate::models::ApiKeyResponseIncludedItem>>,
}

impl ApiKeyResponse {
    /// Response for retrieving an API key.
    pub fn new() -> ApiKeyResponse {
        ApiKeyResponse {
            data: None,
            included: None,
        }
    }
}


