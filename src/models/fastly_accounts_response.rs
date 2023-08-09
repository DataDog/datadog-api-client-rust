/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// FastlyAccountsResponse : The expected response schema when getting Fastly accounts.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FastlyAccountsResponse {
    /// The JSON:API data schema.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::FastlyAccountResponseData>>,
}

impl FastlyAccountsResponse {
    /// The expected response schema when getting Fastly accounts.
    pub fn new() -> FastlyAccountsResponse {
        FastlyAccountsResponse {
            data: None,
        }
    }
}


