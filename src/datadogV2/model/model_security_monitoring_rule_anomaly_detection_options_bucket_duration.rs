// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleAnomalyDetectionOptionsBucketDuration {
    FIVE_MINUTES,
    TEN_MINUTES,
    FIFTEEN_MINUTES,
    THIRTY_MINUTES,
    ONE_HOUR,
    THREE_HOURS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl Serialize for SecurityMonitoringRuleAnomalyDetectionOptionsBucketDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            Self::FIVE_MINUTES => serializer.serialize_i32(300),
            Self::TEN_MINUTES => serializer.serialize_i32(600),
            Self::FIFTEEN_MINUTES => serializer.serialize_i32(900),
            Self::THIRTY_MINUTES => serializer.serialize_i32(1800),
            Self::ONE_HOUR => serializer.serialize_i32(3600),
            Self::THREE_HOURS => serializer.serialize_i32(10800),
        }
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleAnomalyDetectionOptionsBucketDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            300 => Self::FIVE_MINUTES,
            600 => Self::TEN_MINUTES,
            900 => Self::FIFTEEN_MINUTES,
            1800 => Self::THIRTY_MINUTES,
            3600 => Self::ONE_HOUR,
            10800 => Self::THREE_HOURS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::Number(s.into()),
            }),
        })
    }
}
