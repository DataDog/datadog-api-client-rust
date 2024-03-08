// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetTimeWindows {
    SEVEN_DAYS,
    THIRTY_DAYS,
    NINETY_DAYS,
    WEEK_TO_DATE,
    PREVIOUS_WEEK,
    MONTH_TO_DATE,
    PREVIOUS_MONTH,
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

impl Serialize for WidgetTimeWindows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for WidgetTimeWindows {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "7d" => Self::SEVEN_DAYS,
            "30d" => Self::THIRTY_DAYS,
            "90d" => Self::NINETY_DAYS,
            "week_to_date" => Self::WEEK_TO_DATE,
            "previous_week" => Self::PREVIOUS_WEEK,
            "month_to_date" => Self::MONTH_TO_DATE,
            "previous_month" => Self::PREVIOUS_MONTH,
            "global_time" => Self::GLOBAL_TIME,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
