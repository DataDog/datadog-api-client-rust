// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityMonitoringRuleNewValueOptionsForgetAfter {
    #[serde(rename = "1")]
	ONE_DAY,
    #[serde(rename = "2")]
	TWO_DAYS,
    #[serde(rename = "7")]
	ONE_WEEK,
    #[serde(rename = "14")]
	TWO_WEEKS,
    #[serde(rename = "21")]
	THREE_WEEKS,
    #[serde(rename = "28")]
	FOUR_WEEKS,
}

impl ToString for SecurityMonitoringRuleNewValueOptionsForgetAfter {
    fn to_string(&self) -> String {
        match self {
            Self::ONE_DAY => String::from("1"),
            Self::TWO_DAYS => String::from("2"),
            Self::ONE_WEEK => String::from("7"),
            Self::TWO_WEEKS => String::from("14"),
            Self::THREE_WEEKS => String::from("21"),
            Self::FOUR_WEEKS => String::from("28"),
        }
    }
}

impl Default for SecurityMonitoringRuleNewValueOptionsForgetAfter {
    fn default() -> SecurityMonitoringRuleNewValueOptionsForgetAfter {
        Self::ONE_DAY
    }
}
