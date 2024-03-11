// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleMaxSignalDuration {
    ZERO_MINUTES,
    ONE_MINUTE,
    FIVE_MINUTES,
    TEN_MINUTES,
    FIFTEEN_MINUTES,
    THIRTY_MINUTES,
    ONE_HOUR,
    TWO_HOURS,
    THREE_HOURS,
    SIX_HOURS,
    TWELVE_HOURS,
    ONE_DAY,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl Serialize for SecurityMonitoringRuleMaxSignalDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            Self::ZERO_MINUTES => serializer.serialize_i32(0),
            Self::ONE_MINUTE => serializer.serialize_i32(60),
            Self::FIVE_MINUTES => serializer.serialize_i32(300),
            Self::TEN_MINUTES => serializer.serialize_i32(600),
            Self::FIFTEEN_MINUTES => serializer.serialize_i32(900),
            Self::THIRTY_MINUTES => serializer.serialize_i32(1800),
            Self::ONE_HOUR => serializer.serialize_i32(3600),
            Self::TWO_HOURS => serializer.serialize_i32(7200),
            Self::THREE_HOURS => serializer.serialize_i32(10800),
            Self::SIX_HOURS => serializer.serialize_i32(21600),
            Self::TWELVE_HOURS => serializer.serialize_i32(43200),
            Self::ONE_DAY => serializer.serialize_i32(86400),
        }
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleMaxSignalDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => Self::ZERO_MINUTES,
            60 => Self::ONE_MINUTE,
            300 => Self::FIVE_MINUTES,
            600 => Self::TEN_MINUTES,
            900 => Self::FIFTEEN_MINUTES,
            1800 => Self::THIRTY_MINUTES,
            3600 => Self::ONE_HOUR,
            7200 => Self::TWO_HOURS,
            10800 => Self::THREE_HOURS,
            21600 => Self::SIX_HOURS,
            43200 => Self::TWELVE_HOURS,
            86400 => Self::ONE_DAY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::Number(s.into()),
            }),
        })
    }
}
