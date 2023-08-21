// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetEventSize {
    #[serde(rename = "s")]
	SMALL,
    #[serde(rename = "l")]
	LARGE,
}

impl ToString for WidgetEventSize {
    fn to_string(&self) -> String {
        match self {
            Self::SMALL => String::from("s"),
            Self::LARGE => String::from("l"),
        }
    }
}

impl Default for WidgetEventSize {
    fn default() -> WidgetEventSize {
        Self::SMALL
    }
}
