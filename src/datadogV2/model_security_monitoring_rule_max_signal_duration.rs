// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityMonitoringRuleMaxSignalDuration {
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
    #[serde(rename = "10800")]
	THREE_HOURS,
    #[serde(rename = "21600")]
	SIX_HOURS,
    #[serde(rename = "43200")]
	TWELVE_HOURS,
    #[serde(rename = "86400")]
	ONE_DAY,
}

impl ToString for SecurityMonitoringRuleMaxSignalDuration {
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
            Self::THREE_HOURS => String::from("10800"),
            Self::SIX_HOURS => String::from("21600"),
            Self::TWELVE_HOURS => String::from("43200"),
            Self::ONE_DAY => String::from("86400"),
        }
    }
}

impl Default for SecurityMonitoringRuleMaxSignalDuration {
    fn default() -> SecurityMonitoringRuleMaxSignalDuration {
        Self::ZERO_MINUTES
    }
}
