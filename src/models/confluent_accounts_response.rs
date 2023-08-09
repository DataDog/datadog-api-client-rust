/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ConfluentAccountsResponse : Confluent account returned by the API.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfluentAccountsResponse {
    /// The Confluent account.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::ConfluentAccountResponseData>>,
}

impl ConfluentAccountsResponse {
    /// Confluent account returned by the API.
    pub fn new() -> ConfluentAccountsResponse {
        ConfluentAccountsResponse {
            data: None,
        }
    }
}


