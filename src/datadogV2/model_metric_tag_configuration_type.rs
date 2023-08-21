// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetricTagConfigurationType {
    #[serde(rename = "manage_tags")]
	MANAGE_TAGS,
}

impl ToString for MetricTagConfigurationType {
    fn to_string(&self) -> String {
        match self {
            Self::MANAGE_TAGS => String::from("manage_tags"),
        }
    }
}

impl Default for MetricTagConfigurationType {
    fn default() -> MetricTagConfigurationType {
        Self::MANAGE_TAGS
    }
}
