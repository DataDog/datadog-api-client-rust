/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SpansMetricCompute : The compute rule to compute the span-based metric.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpansMetricCompute {
    #[serde(rename = "aggregation_type")]
    pub aggregation_type: crate::models::SpansMetricComputeAggregationType,
    /// Toggle to include or exclude percentile aggregations for distribution metrics. Only present when the `aggregation_type` is `distribution`.
    #[serde(rename = "include_percentiles", skip_serializing_if = "Option::is_none")]
    pub include_percentiles: Option<bool>,
    /// The path to the value the span-based metric will aggregate on (only used if the aggregation type is a \"distribution\").
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl SpansMetricCompute {
    /// The compute rule to compute the span-based metric.
    pub fn new(aggregation_type: crate::models::SpansMetricComputeAggregationType) -> SpansMetricCompute {
        SpansMetricCompute {
            aggregation_type,
            include_percentiles: None,
            path: None,
        }
    }
}


