// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceCheckStatus {
    #[serde(rename = "0")]
	OK,
    #[serde(rename = "1")]
	WARNING,
    #[serde(rename = "2")]
	CRITICAL,
    #[serde(rename = "3")]
	UNKNOWN,
}

impl ToString for ServiceCheckStatus {
    fn to_string(&self) -> String {
        match self {
            Self::OK => String::from("0"),
            Self::WARNING => String::from("1"),
            Self::CRITICAL => String::from("2"),
            Self::UNKNOWN => String::from("3"),
        }
    }
}

impl Default for ServiceCheckStatus {
    fn default() -> ServiceCheckStatus {
        Self::OK
    }
}
