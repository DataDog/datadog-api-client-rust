// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityMonitoringRuleNewValueOptionsLearningMethod {
    #[serde(rename = "duration")]
	DURATION,
    #[serde(rename = "threshold")]
	THRESHOLD,
}

impl ToString for SecurityMonitoringRuleNewValueOptionsLearningMethod {
    fn to_string(&self) -> String {
        match self {
            Self::DURATION => String::from("duration"),
            Self::THRESHOLD => String::from("threshold"),
        }
    }
}

impl Default for SecurityMonitoringRuleNewValueOptionsLearningMethod {
    fn default() -> SecurityMonitoringRuleNewValueOptionsLearningMethod {
        Self::DURATION
    }
}
