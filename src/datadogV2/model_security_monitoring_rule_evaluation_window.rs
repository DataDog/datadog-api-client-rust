// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityMonitoringRuleEvaluationWindow {
    #[serde(rename = "0")]
	ZERO_MINUTES,
    #[serde(rename = "60")]
	ONE_MINUTE,
    #[serde(rename = "300")]
	FIVE_MINUTES,
    #[serde(rename = "600")]
	TEN_MINUTES,
    #[serde(rename = "900")]
	FIFTEEN_MINUTES,
    #[serde(rename = "1800")]
	THIRTY_MINUTES,
    #[serde(rename = "3600")]
	ONE_HOUR,
    #[serde(rename = "7200")]
	TWO_HOURS,
}

impl ToString for SecurityMonitoringRuleEvaluationWindow {
    fn to_string(&self) -> String {
        match self {
            Self::ZERO_MINUTES => String::from("0"),
            Self::ONE_MINUTE => String::from("60"),
            Self::FIVE_MINUTES => String::from("300"),
            Self::TEN_MINUTES => String::from("600"),
            Self::FIFTEEN_MINUTES => String::from("900"),
            Self::THIRTY_MINUTES => String::from("1800"),
            Self::ONE_HOUR => String::from("3600"),
            Self::TWO_HOURS => String::from("7200"),
        }
    }
}

impl Default for SecurityMonitoringRuleEvaluationWindow {
    fn default() -> SecurityMonitoringRuleEvaluationWindow {
        Self::ZERO_MINUTES
    }
}
