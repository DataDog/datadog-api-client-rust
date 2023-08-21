// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetCompareTo {
    #[serde(rename = "hour_before")]
	HOUR_BEFORE,
    #[serde(rename = "day_before")]
	DAY_BEFORE,
    #[serde(rename = "week_before")]
	WEEK_BEFORE,
    #[serde(rename = "month_before")]
	MONTH_BEFORE,
}

impl ToString for WidgetCompareTo {
    fn to_string(&self) -> String {
        match self {
            Self::HOUR_BEFORE => String::from("hour_before"),
            Self::DAY_BEFORE => String::from("day_before"),
            Self::WEEK_BEFORE => String::from("week_before"),
            Self::MONTH_BEFORE => String::from("month_before"),
        }
    }
}

impl Default for WidgetCompareTo {
    fn default() -> WidgetCompareTo {
        Self::HOUR_BEFORE
    }
}
