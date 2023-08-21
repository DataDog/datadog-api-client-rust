// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsTestPauseStatus {
    #[serde(rename = "live")]
	LIVE,
    #[serde(rename = "paused")]
	PAUSED,
}

impl ToString for SyntheticsTestPauseStatus {
    fn to_string(&self) -> String {
        match self {
            Self::LIVE => String::from("live"),
            Self::PAUSED => String::from("paused"),
        }
    }
}

impl Default for SyntheticsTestPauseStatus {
    fn default() -> SyntheticsTestPauseStatus {
        Self::LIVE
    }
}
