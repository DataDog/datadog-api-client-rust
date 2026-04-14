// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FunnelComparisonDurationType {
    PREVIOUS_TIMEFRAME,
    CUSTOM_TIMEFRAME,
    PREVIOUS_DAY,
    PREVIOUS_WEEK,
    PREVIOUS_MONTH,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for FunnelComparisonDurationType {
    fn to_string(&self) -> String {
        match self {
            Self::PREVIOUS_TIMEFRAME => String::from("previous_timeframe"),
            Self::CUSTOM_TIMEFRAME => String::from("custom_timeframe"),
            Self::PREVIOUS_DAY => String::from("previous_day"),
            Self::PREVIOUS_WEEK => String::from("previous_week"),
            Self::PREVIOUS_MONTH => String::from("previous_month"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FunnelComparisonDurationType {
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

impl<'de> Deserialize<'de> for FunnelComparisonDurationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "previous_timeframe" => Self::PREVIOUS_TIMEFRAME,
            "custom_timeframe" => Self::CUSTOM_TIMEFRAME,
            "previous_day" => Self::PREVIOUS_DAY,
            "previous_week" => Self::PREVIOUS_WEEK,
            "previous_month" => Self::PREVIOUS_MONTH,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
