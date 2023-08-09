/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SpansAggregateResponseMetadata : The metadata associated with a request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpansAggregateResponseMetadata {
    /// The time elapsed in milliseconds.
    #[serde(rename = "elapsed", skip_serializing_if = "Option::is_none")]
    pub elapsed: Option<i64>,
    /// The identifier of the request.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::SpansAggregateResponseStatus>,
    /// A list of warnings (non fatal errors) encountered, partial results might be returned if warnings are present in the response.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<crate::models::SpansWarning>>,
}

impl SpansAggregateResponseMetadata {
    /// The metadata associated with a request.
    pub fn new() -> SpansAggregateResponseMetadata {
        SpansAggregateResponseMetadata {
            elapsed: None,
            request_id: None,
            status: None,
            warnings: None,
        }
    }
}


