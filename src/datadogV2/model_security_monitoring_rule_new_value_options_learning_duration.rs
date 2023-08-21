// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityMonitoringRuleNewValueOptionsLearningDuration {
    #[serde(rename = "0")]
	ZERO_DAYS,
    #[serde(rename = "1")]
	ONE_DAY,
    #[serde(rename = "7")]
	SEVEN_DAYS,
}

impl ToString for SecurityMonitoringRuleNewValueOptionsLearningDuration {
    fn to_string(&self) -> String {
        match self {
            Self::ZERO_DAYS => String::from("0"),
            Self::ONE_DAY => String::from("1"),
            Self::SEVEN_DAYS => String::from("7"),
        }
    }
}

impl Default for SecurityMonitoringRuleNewValueOptionsLearningDuration {
    fn default() -> SecurityMonitoringRuleNewValueOptionsLearningDuration {
        Self::ZERO_DAYS
    }
}
