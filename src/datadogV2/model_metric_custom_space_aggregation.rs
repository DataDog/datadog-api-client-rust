// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetricCustomSpaceAggregation {
    #[serde(rename = "avg")]
	AVG,
    #[serde(rename = "max")]
	MAX,
    #[serde(rename = "min")]
	MIN,
    #[serde(rename = "sum")]
	SUM,
}

impl ToString for MetricCustomSpaceAggregation {
    fn to_string(&self) -> String {
        match self {
            Self::AVG => String::from("avg"),
            Self::MAX => String::from("max"),
            Self::MIN => String::from("min"),
            Self::SUM => String::from("sum"),
        }
    }
}

impl Default for MetricCustomSpaceAggregation {
    fn default() -> MetricCustomSpaceAggregation {
        Self::AVG
    }
}
