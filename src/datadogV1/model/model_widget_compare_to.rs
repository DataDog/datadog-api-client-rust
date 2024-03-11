// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetCompareTo {
    HOUR_BEFORE,
    DAY_BEFORE,
    WEEK_BEFORE,
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

impl Serialize for WidgetCompareTo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for WidgetCompareTo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "hour_before" => Self::HOUR_BEFORE,
            "day_before" => Self::DAY_BEFORE,
            "week_before" => Self::WEEK_BEFORE,
            "month_before" => Self::MONTH_BEFORE,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
