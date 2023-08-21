// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsAPIStepSubtype {
    #[serde(rename = "http")]
	HTTP,
}

impl ToString for SyntheticsAPIStepSubtype {
    fn to_string(&self) -> String {
        match self {
            Self::HTTP => String::from("http"),
        }
    }
}

impl Default for SyntheticsAPIStepSubtype {
    fn default() -> SyntheticsAPIStepSubtype {
        Self::HTTP
    }
}
