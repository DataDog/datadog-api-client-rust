// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the definition of a metric's ingested and indexed volume.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricIngestedIndexedVolumeAttributes {
    /// Indexed volume for the given metric.
    #[serde(rename = "indexed_volume")]
    pub indexed_volume: Option<i64>,
    /// Ingested volume for the given metric.
    #[serde(rename = "ingested_volume")]
    pub ingested_volume: Option<i64>,
}

impl MetricIngestedIndexedVolumeAttributes {
    pub fn new() -> MetricIngestedIndexedVolumeAttributes {
        MetricIngestedIndexedVolumeAttributes {
            indexed_volume: None,
            ingested_volume: None,
        }
    }
}
impl Default for MetricIngestedIndexedVolumeAttributes {
    fn default() -> Self {
        Self::new()
    }
}
