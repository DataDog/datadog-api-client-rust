// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object listing all metric names stored by Datadog since a given time.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsListResponse {
    /// Time when the metrics were active, seconds since the Unix epoch.
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// List of metric names.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<String>>,
}

impl MetricsListResponse {
    pub fn new() -> MetricsListResponse {
        MetricsListResponse {
            from: None,
            metrics: None,
        }
    }

    pub fn with_from(&mut self, value: String) -> &mut Self {
        self.from = Some(value);
        self
    }

    pub fn with_metrics(&mut self, value: Vec<String>) -> &mut Self {
        self.metrics = Some(value);
        self
    }
}
impl Default for MetricsListResponse {
    fn default() -> Self {
        Self::new()
    }
}
