// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetricIngestedIndexedVolumeType {
    #[serde(rename = "metric_volumes")]
	METRIC_VOLUMES,
}

impl ToString for MetricIngestedIndexedVolumeType {
    fn to_string(&self) -> String {
        match self {
            Self::METRIC_VOLUMES => String::from("metric_volumes"),
        }
    }
}

impl Default for MetricIngestedIndexedVolumeType {
    fn default() -> MetricIngestedIndexedVolumeType {
        Self::METRIC_VOLUMES
    }
}
