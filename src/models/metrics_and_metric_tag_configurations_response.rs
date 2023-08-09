/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MetricsAndMetricTagConfigurationsResponse : Response object that includes metrics and metric tag configurations.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetricsAndMetricTagConfigurationsResponse {
    /// Array of metrics and metric tag configurations.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::MetricsAndMetricTagConfigurations>>,
}

impl MetricsAndMetricTagConfigurationsResponse {
    /// Response object that includes metrics and metric tag configurations.
    pub fn new() -> MetricsAndMetricTagConfigurationsResponse {
        MetricsAndMetricTagConfigurationsResponse {
            data: None,
        }
    }
}


