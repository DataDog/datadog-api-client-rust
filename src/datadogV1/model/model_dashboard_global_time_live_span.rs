// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DashboardGlobalTimeLiveSpan {
    PAST_FIFTEEN_MINUTES,
    PAST_ONE_HOUR,
    PAST_FOUR_HOURS,
    PAST_ONE_DAY,
    PAST_TWO_DAYS,
    PAST_ONE_WEEK,
    PAST_ONE_MONTH,
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

impl Serialize for DashboardGlobalTimeLiveSpan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for DashboardGlobalTimeLiveSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "15m" => Self::PAST_FIFTEEN_MINUTES,
            "1h" => Self::PAST_ONE_HOUR,
            "4h" => Self::PAST_FOUR_HOURS,
            "1d" => Self::PAST_ONE_DAY,
            "2d" => Self::PAST_TWO_DAYS,
            "1w" => Self::PAST_ONE_WEEK,
            "1mo" => Self::PAST_ONE_MONTH,
            "3mo" => Self::PAST_THREE_MONTHS,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
