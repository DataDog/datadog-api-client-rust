/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsMetricResponseCompute : The compute rule to compute the log-based metric.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogsMetricResponseCompute {
    #[serde(rename = "aggregation_type", skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<crate::models::LogsMetricResponseComputeAggregationType>,
    /// Toggle to include or exclude percentile aggregations for distribution metrics. Only present when the `aggregation_type` is `distribution`.
    #[serde(rename = "include_percentiles", skip_serializing_if = "Option::is_none")]
    pub include_percentiles: Option<bool>,
    /// The path to the value the log-based metric will aggregate on (only used if the aggregation type is a \"distribution\").
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl LogsMetricResponseCompute {
    /// The compute rule to compute the log-based metric.
    pub fn new() -> LogsMetricResponseCompute {
        LogsMetricResponseCompute {
            aggregation_type: None,
            include_percentiles: None,
            path: None,
        }
    }
}


