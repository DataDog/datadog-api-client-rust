// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsTestMonitorStatus {
    #[serde(rename = "0")]
	UNTRIGGERED,
    #[serde(rename = "1")]
	TRIGGERED,
    #[serde(rename = "2")]
	NO_DATA,
}

impl ToString for SyntheticsTestMonitorStatus {
    fn to_string(&self) -> String {
        match self {
            Self::UNTRIGGERED => String::from("0"),
            Self::TRIGGERED => String::from("1"),
            Self::NO_DATA => String::from("2"),
        }
    }
}

impl Default for SyntheticsTestMonitorStatus {
    fn default() -> SyntheticsTestMonitorStatus {
        Self::UNTRIGGERED
    }
}
