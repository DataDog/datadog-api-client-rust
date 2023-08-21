// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetOrderBy {
    #[serde(rename = "change")]
	CHANGE,
    #[serde(rename = "name")]
	NAME,
    #[serde(rename = "present")]
	PRESENT,
    #[serde(rename = "past")]
	PAST,
}

impl ToString for WidgetOrderBy {
    fn to_string(&self) -> String {
        match self {
            Self::CHANGE => String::from("change"),
            Self::NAME => String::from("name"),
            Self::PRESENT => String::from("present"),
            Self::PAST => String::from("past"),
        }
    }
}

impl Default for WidgetOrderBy {
    fn default() -> WidgetOrderBy {
        Self::CHANGE
    }
}
