// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UsageSortDirection {
    #[serde(rename = "desc")]
	DESC,
    #[serde(rename = "asc")]
	ASC,
}

impl ToString for UsageSortDirection {
    fn to_string(&self) -> String {
        match self {
            Self::DESC => String::from("desc"),
            Self::ASC => String::from("asc"),
        }
    }
}

impl Default for UsageSortDirection {
    fn default() -> UsageSortDirection {
        Self::DESC
    }
}
