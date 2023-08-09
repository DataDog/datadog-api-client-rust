/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsMetricType : The type of the resource. The value should always be logs_metrics.

/// The type of the resource. The value should always be logs_metrics.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsMetricType {
    #[serde(rename = "logs_metrics")]
    LOGS_METRICS,

}

impl ToString for LogsMetricType {
    fn to_string(&self) -> String {
        match self {
            Self::LOGS_METRICS => String::from("logs_metrics"),
        }
    }
}

impl Default for LogsMetricType {
    fn default() -> LogsMetricType {
        Self::LOGS_METRICS
    }
}




