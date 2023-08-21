// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetSizeFormat {
    #[serde(rename = "small")]
	SMALL,
    #[serde(rename = "medium")]
	MEDIUM,
    #[serde(rename = "large")]
	LARGE,
}

impl ToString for WidgetSizeFormat {
    fn to_string(&self) -> String {
        match self {
            Self::SMALL => String::from("small"),
            Self::MEDIUM => String::from("medium"),
            Self::LARGE => String::from("large"),
        }
    }
}

impl Default for WidgetSizeFormat {
    fn default() -> WidgetSizeFormat {
        Self::SMALL
    }
}
