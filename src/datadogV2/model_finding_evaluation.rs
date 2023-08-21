// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FindingEvaluation {
    #[serde(rename = "pass")]
	PASS,
    #[serde(rename = "fail")]
	FAIL,
}

impl ToString for FindingEvaluation {
    fn to_string(&self) -> String {
        match self {
            Self::PASS => String::from("pass"),
            Self::FAIL => String::from("fail"),
        }
    }
}

impl Default for FindingEvaluation {
    fn default() -> FindingEvaluation {
        Self::PASS
    }
}
