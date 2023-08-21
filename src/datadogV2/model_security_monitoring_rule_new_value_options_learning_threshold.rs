// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityMonitoringRuleNewValueOptionsLearningThreshold {
    #[serde(rename = "0")]
	ZERO_OCCURRENCES,
    #[serde(rename = "1")]
	ONE_OCCURRENCE,
}

impl ToString for SecurityMonitoringRuleNewValueOptionsLearningThreshold {
    fn to_string(&self) -> String {
        match self {
            Self::ZERO_OCCURRENCES => String::from("0"),
            Self::ONE_OCCURRENCE => String::from("1"),
        }
    }
}

impl Default for SecurityMonitoringRuleNewValueOptionsLearningThreshold {
    fn default() -> SecurityMonitoringRuleNewValueOptionsLearningThreshold {
        Self::ZERO_OCCURRENCES
    }
}
