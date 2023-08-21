// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
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
