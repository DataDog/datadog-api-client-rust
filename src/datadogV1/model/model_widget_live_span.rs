// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetLiveSpan {
    PAST_ONE_MINUTE,
    PAST_FIVE_MINUTES,
    PAST_TEN_MINUTES,
    PAST_FIFTEEN_MINUTES,
    PAST_THIRTY_MINUTES,
    PAST_ONE_HOUR,
    PAST_FOUR_HOURS,
    PAST_ONE_DAY,
    PAST_TWO_DAYS,
    PAST_ONE_WEEK,
    PAST_ONE_MONTH,
    PAST_THREE_MONTHS,
    PAST_SIX_MONTHS,
    WEEK_TO_DATE,
    MONTH_TO_DATE,
    PAST_ONE_YEAR,
    ALERT,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for WidgetLiveSpan {
    fn to_string(&self) -> String {
        match self {
            Self::PAST_ONE_MINUTE => String::from("1m"),
            Self::PAST_FIVE_MINUTES => String::from("5m"),
            Self::PAST_TEN_MINUTES => String::from("10m"),
            Self::PAST_FIFTEEN_MINUTES => String::from("15m"),
            Self::PAST_THIRTY_MINUTES => String::from("30m"),
            Self::PAST_ONE_HOUR => String::from("1h"),
            Self::PAST_FOUR_HOURS => String::from("4h"),
            Self::PAST_ONE_DAY => String::from("1d"),
            Self::PAST_TWO_DAYS => String::from("2d"),
            Self::PAST_ONE_WEEK => String::from("1w"),
            Self::PAST_ONE_MONTH => String::from("1mo"),
            Self::PAST_THREE_MONTHS => String::from("3mo"),
            Self::PAST_SIX_MONTHS => String::from("6mo"),
            Self::WEEK_TO_DATE => String::from("week_to_date"),
            Self::MONTH_TO_DATE => String::from("month_to_date"),
            Self::PAST_ONE_YEAR => String::from("1y"),
            Self::ALERT => String::from("alert"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for WidgetLiveSpan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for WidgetLiveSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "1m" => Self::PAST_ONE_MINUTE,
            "5m" => Self::PAST_FIVE_MINUTES,
            "10m" => Self::PAST_TEN_MINUTES,
            "15m" => Self::PAST_FIFTEEN_MINUTES,
            "30m" => Self::PAST_THIRTY_MINUTES,
            "1h" => Self::PAST_ONE_HOUR,
            "4h" => Self::PAST_FOUR_HOURS,
            "1d" => Self::PAST_ONE_DAY,
            "2d" => Self::PAST_TWO_DAYS,
            "1w" => Self::PAST_ONE_WEEK,
            "1mo" => Self::PAST_ONE_MONTH,
            "3mo" => Self::PAST_THREE_MONTHS,
            "6mo" => Self::PAST_SIX_MONTHS,
            "week_to_date" => Self::WEEK_TO_DATE,
            "month_to_date" => Self::MONTH_TO_DATE,
            "1y" => Self::PAST_ONE_YEAR,
            "alert" => Self::ALERT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
