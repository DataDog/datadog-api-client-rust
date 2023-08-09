/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MetricType : The metric resource type.

/// The metric resource type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetricType {
    #[serde(rename = "metrics")]
    METRICS,

}

impl ToString for MetricType {
    fn to_string(&self) -> String {
        match self {
            Self::METRICS => String::from("metrics"),
        }
    }
}

impl Default for MetricType {
    fn default() -> MetricType {
        Self::METRICS
    }
}




