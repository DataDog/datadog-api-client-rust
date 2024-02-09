// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object with all metric related metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricMetadata {
    /// Metric description.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Name of the integration that sent the metric if applicable.
    #[serde(rename = "integration")]
    pub integration: Option<String>,
    /// Per unit of the metric such as `second` in `bytes per second`.
    #[serde(rename = "per_unit")]
    pub per_unit: Option<String>,
    /// A more human-readable and abbreviated version of the metric name.
    #[serde(rename = "short_name")]
    pub short_name: Option<String>,
    /// StatsD flush interval of the metric in seconds if applicable.
    #[serde(rename = "statsd_interval")]
    pub statsd_interval: Option<i64>,
    /// Metric type such as `gauge` or `rate`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Primary unit of the metric such as `byte` or `operation`.
    #[serde(rename = "unit")]
    pub unit: Option<String>,
}

impl MetricMetadata {
    pub fn new() -> MetricMetadata {
        MetricMetadata {
            description: None,
            integration: None,
            per_unit: None,
            short_name: None,
            statsd_interval: None,
            type_: None,
            unit: None,
        }
    }

    pub fn description(&mut self, value: String) -> &mut Self {
        self.description = Some(value);
        self
    }

    pub fn integration(&mut self, value: String) -> &mut Self {
        self.integration = Some(value);
        self
    }

    pub fn per_unit(&mut self, value: String) -> &mut Self {
        self.per_unit = Some(value);
        self
    }

    pub fn short_name(&mut self, value: String) -> &mut Self {
        self.short_name = Some(value);
        self
    }

    pub fn statsd_interval(&mut self, value: i64) -> &mut Self {
        self.statsd_interval = Some(value);
        self
    }

    pub fn type_(&mut self, value: String) -> &mut Self {
        self.type_ = Some(value);
        self
    }

    pub fn unit(&mut self, value: String) -> &mut Self {
        self.unit = Some(value);
        self
    }
}

impl Default for MetricMetadata {
    fn default() -> Self {
        Self::new()
    }
}
