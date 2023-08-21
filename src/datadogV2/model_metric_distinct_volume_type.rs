// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetricDistinctVolumeType {
    #[serde(rename = "distinct_metric_volumes")]
	DISTINCT_METRIC_VOLUMES,
}

impl ToString for MetricDistinctVolumeType {
    fn to_string(&self) -> String {
        match self {
            Self::DISTINCT_METRIC_VOLUMES => String::from("distinct_metric_volumes"),
        }
    }
}

impl Default for MetricDistinctVolumeType {
    fn default() -> MetricDistinctVolumeType {
        Self::DISTINCT_METRIC_VOLUMES
    }
}
