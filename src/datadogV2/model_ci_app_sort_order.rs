// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CIAppSortOrder {
    #[serde(rename = "asc")]
	ASCENDING,
    #[serde(rename = "desc")]
	DESCENDING,
}

impl ToString for CIAppSortOrder {
    fn to_string(&self) -> String {
        match self {
            Self::ASCENDING => String::from("asc"),
            Self::DESCENDING => String::from("desc"),
        }
    }
}

impl Default for CIAppSortOrder {
    fn default() -> CIAppSortOrder {
        Self::ASCENDING
    }
}
