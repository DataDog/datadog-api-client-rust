// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityMonitoringRuleSeverity {
    #[serde(rename = "info")]
	INFO,
    #[serde(rename = "low")]
	LOW,
    #[serde(rename = "medium")]
	MEDIUM,
    #[serde(rename = "high")]
	HIGH,
    #[serde(rename = "critical")]
	CRITICAL,
}

impl ToString for SecurityMonitoringRuleSeverity {
    fn to_string(&self) -> String {
        match self {
            Self::INFO => String::from("info"),
            Self::LOW => String::from("low"),
            Self::MEDIUM => String::from("medium"),
            Self::HIGH => String::from("high"),
            Self::CRITICAL => String::from("critical"),
        }
    }
}

impl Default for SecurityMonitoringRuleSeverity {
    fn default() -> SecurityMonitoringRuleSeverity {
        Self::INFO
    }
}
