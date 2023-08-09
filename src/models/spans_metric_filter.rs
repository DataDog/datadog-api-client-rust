/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SpansMetricFilter : The span-based metric filter. Spans matching this filter will be aggregated in this metric.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpansMetricFilter {
    /// The search query - following the span search syntax.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl SpansMetricFilter {
    /// The span-based metric filter. Spans matching this filter will be aggregated in this metric.
    pub fn new() -> SpansMetricFilter {
        SpansMetricFilter {
            query: None,
        }
    }
}


