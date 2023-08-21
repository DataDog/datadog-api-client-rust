// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FindingType {
    #[serde(rename = "finding")]
	FINDING,
}

impl ToString for FindingType {
    fn to_string(&self) -> String {
        match self {
            Self::FINDING => String::from("finding"),
        }
    }
}

impl Default for FindingType {
    fn default() -> FindingType {
        Self::FINDING
    }
}
