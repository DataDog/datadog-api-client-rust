// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotifyEndState {
    #[serde(rename = "alert")]
	ALERT,
    #[serde(rename = "no data")]
	NO_DATA,
    #[serde(rename = "warn")]
	WARN,
}

impl ToString for NotifyEndState {
    fn to_string(&self) -> String {
        match self {
            Self::ALERT => String::from("alert"),
            Self::NO_DATA => String::from("no data"),
            Self::WARN => String::from("warn"),
        }
    }
}

impl Default for NotifyEndState {
    fn default() -> NotifyEndState {
        Self::ALERT
    }
}
