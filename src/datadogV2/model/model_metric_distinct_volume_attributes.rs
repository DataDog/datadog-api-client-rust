// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the definition of a metric's distinct volume.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDistinctVolumeAttributes {
    /// Distinct volume for the given metric.
    #[serde(rename = "distinct_volume")]
    pub distinct_volume: Option<i64>,
}

impl MetricDistinctVolumeAttributes {
    pub fn new() -> MetricDistinctVolumeAttributes {
        MetricDistinctVolumeAttributes {
            distinct_volume: None,
        }
    }

    pub fn distinct_volume(&mut self, value: i64) -> &mut Self {
        self.distinct_volume = Some(value);
        self
    }
}

impl Default for MetricDistinctVolumeAttributes {
    fn default() -> Self {
        Self::new()
    }
}
