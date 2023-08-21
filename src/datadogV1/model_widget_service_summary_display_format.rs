// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetServiceSummaryDisplayFormat {
    #[serde(rename = "one_column")]
	ONE_COLUMN,
    #[serde(rename = "two_column")]
	TWO_COLUMN,
    #[serde(rename = "three_column")]
	THREE_COLUMN,
}

impl ToString for WidgetServiceSummaryDisplayFormat {
    fn to_string(&self) -> String {
        match self {
            Self::ONE_COLUMN => String::from("one_column"),
            Self::TWO_COLUMN => String::from("two_column"),
            Self::THREE_COLUMN => String::from("three_column"),
        }
    }
}

impl Default for WidgetServiceSummaryDisplayFormat {
    fn default() -> WidgetServiceSummaryDisplayFormat {
        Self::ONE_COLUMN
    }
}
