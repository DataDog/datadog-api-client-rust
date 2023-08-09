/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SpansAggregateResponse : The response object for the spans aggregate API endpoint.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpansAggregateResponse {
    /// The list of matching buckets, one item per bucket.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::SpansAggregateBucket>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::SpansAggregateResponseMetadata>>,
}

impl SpansAggregateResponse {
    /// The response object for the spans aggregate API endpoint.
    pub fn new() -> SpansAggregateResponse {
        SpansAggregateResponse {
            data: None,
            meta: None,
        }
    }
}


