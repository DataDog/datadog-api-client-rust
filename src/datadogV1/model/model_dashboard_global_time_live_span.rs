// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DashboardGlobalTimeLiveSpan {
    #[serde(rename = "15m")]
    PAST_FIFTEEN_MINUTES,
    #[serde(rename = "1h")]
    PAST_ONE_HOUR,
    #[serde(rename = "4h")]
    PAST_FOUR_HOURS,
    #[serde(rename = "1d")]
    PAST_ONE_DAY,
    #[serde(rename = "2d")]
    PAST_TWO_DAYS,
    #[serde(rename = "1w")]
    PAST_ONE_WEEK,
    #[serde(rename = "1mo")]
    PAST_ONE_MONTH,
    #[serde(rename = "3mo")]
    PAST_THREE_MONTHS,
}

impl ToString for DashboardGlobalTimeLiveSpan {
    fn to_string(&self) -> String {
        match self {
            Self::PAST_FIFTEEN_MINUTES => String::from("15m"),
            Self::PAST_ONE_HOUR => String::from("1h"),
            Self::PAST_FOUR_HOURS => String::from("4h"),
            Self::PAST_ONE_DAY => String::from("1d"),
            Self::PAST_TWO_DAYS => String::from("2d"),
            Self::PAST_ONE_WEEK => String::from("1w"),
            Self::PAST_ONE_MONTH => String::from("1mo"),
            Self::PAST_THREE_MONTHS => String::from("3mo"),
        }
    }
}
