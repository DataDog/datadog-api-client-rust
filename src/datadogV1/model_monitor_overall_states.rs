// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MonitorOverallStates {
    #[serde(rename = "Alert")]
	ALERT,
    #[serde(rename = "Ignored")]
	IGNORED,
    #[serde(rename = "No Data")]
	NO_DATA,
    #[serde(rename = "OK")]
	OK,
    #[serde(rename = "Skipped")]
	SKIPPED,
    #[serde(rename = "Unknown")]
	UNKNOWN,
    #[serde(rename = "Warn")]
	WARN,
}

impl ToString for MonitorOverallStates {
    fn to_string(&self) -> String {
        match self {
            Self::ALERT => String::from("Alert"),
            Self::IGNORED => String::from("Ignored"),
            Self::NO_DATA => String::from("No Data"),
            Self::OK => String::from("OK"),
            Self::SKIPPED => String::from("Skipped"),
            Self::UNKNOWN => String::from("Unknown"),
            Self::WARN => String::from("Warn"),
        }
    }
}

impl Default for MonitorOverallStates {
    fn default() -> MonitorOverallStates {
        Self::ALERT
    }
}
