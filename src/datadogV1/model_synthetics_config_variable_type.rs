// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsConfigVariableType {
    #[serde(rename = "global")]
	GLOBAL,
    #[serde(rename = "text")]
	TEXT,
}

impl ToString for SyntheticsConfigVariableType {
    fn to_string(&self) -> String {
        match self {
            Self::GLOBAL => String::from("global"),
            Self::TEXT => String::from("text"),
        }
    }
}

impl Default for SyntheticsConfigVariableType {
    fn default() -> SyntheticsConfigVariableType {
        Self::GLOBAL
    }
}
