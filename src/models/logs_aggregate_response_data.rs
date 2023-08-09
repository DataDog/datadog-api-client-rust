/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsAggregateResponseData : The query results



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogsAggregateResponseData {
    /// The list of matching buckets, one item per bucket
    #[serde(rename = "buckets", skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<crate::models::LogsAggregateBucket>>,
}

impl LogsAggregateResponseData {
    /// The query results
    pub fn new() -> LogsAggregateResponseData {
        LogsAggregateResponseData {
            buckets: None,
        }
    }
}


