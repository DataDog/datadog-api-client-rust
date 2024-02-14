// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetTimeWindows {
    #[serde(rename = "7d")]
    SEVEN_DAYS,
    #[serde(rename = "30d")]
    THIRTY_DAYS,
    #[serde(rename = "90d")]
    NINETY_DAYS,
    #[serde(rename = "week_to_date")]
    WEEK_TO_DATE,
    #[serde(rename = "previous_week")]
    PREVIOUS_WEEK,
    #[serde(rename = "month_to_date")]
    MONTH_TO_DATE,
    #[serde(rename = "previous_month")]
    PREVIOUS_MONTH,
    #[serde(rename = "global_time")]
    GLOBAL_TIME,
}

impl ToString for WidgetTimeWindows {
    fn to_string(&self) -> String {
        match self {
            Self::SEVEN_DAYS => String::from("7d"),
            Self::THIRTY_DAYS => String::from("30d"),
            Self::NINETY_DAYS => String::from("90d"),
            Self::WEEK_TO_DATE => String::from("week_to_date"),
            Self::PREVIOUS_WEEK => String::from("previous_week"),
            Self::MONTH_TO_DATE => String::from("month_to_date"),
            Self::PREVIOUS_MONTH => String::from("previous_month"),
            Self::GLOBAL_TIME => String::from("global_time"),
        }
    }
}
