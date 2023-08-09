/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MetricTagConfigurationMetricTypes : The metric's type.

/// The metric's type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetricTagConfigurationMetricTypes {
    #[serde(rename = "gauge")]
    GAUGE,
    #[serde(rename = "count")]
    COUNT,
    #[serde(rename = "rate")]
    RATE,
    #[serde(rename = "distribution")]
    DISTRIBUTION,

}

impl ToString for MetricTagConfigurationMetricTypes {
    fn to_string(&self) -> String {
        match self {
            Self::GAUGE => String::from("gauge"),
            Self::COUNT => String::from("count"),
            Self::RATE => String::from("rate"),
            Self::DISTRIBUTION => String::from("distribution"),
        }
    }
}

impl Default for MetricTagConfigurationMetricTypes {
    fn default() -> MetricTagConfigurationMetricTypes {
        Self::GAUGE
    }
}




