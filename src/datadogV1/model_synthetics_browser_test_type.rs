// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsBrowserTestType {
    #[serde(rename = "browser")]
	BROWSER,
}

impl ToString for SyntheticsBrowserTestType {
    fn to_string(&self) -> String {
        match self {
            Self::BROWSER => String::from("browser"),
        }
    }
}

impl Default for SyntheticsBrowserTestType {
    fn default() -> SyntheticsBrowserTestType {
        Self::BROWSER
    }
}
