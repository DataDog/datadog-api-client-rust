// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsTestExecutionRule {
    #[serde(rename = "blocking")]
	BLOCKING,
    #[serde(rename = "non_blocking")]
	NON_BLOCKING,
    #[serde(rename = "skipped")]
	SKIPPED,
}

impl ToString for SyntheticsTestExecutionRule {
    fn to_string(&self) -> String {
        match self {
            Self::BLOCKING => String::from("blocking"),
            Self::NON_BLOCKING => String::from("non_blocking"),
            Self::SKIPPED => String::from("skipped"),
        }
    }
}

impl Default for SyntheticsTestExecutionRule {
    fn default() -> SyntheticsTestExecutionRule {
        Self::BLOCKING
    }
}
