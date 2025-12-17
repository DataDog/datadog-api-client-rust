// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleAnomalyDetectionOptionsLearningDuration {
    ONE_HOUR,
    SIX_HOURS,
    TWELVE_HOURS,
    ONE_DAY,
    TWO_DAYS,
    ONE_WEEK,
    TWO_WEEKS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl Serialize for SecurityMonitoringRuleAnomalyDetectionOptionsLearningDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            Self::ONE_HOUR => serializer.serialize_i32(1),
            Self::SIX_HOURS => serializer.serialize_i32(6),
            Self::TWELVE_HOURS => serializer.serialize_i32(12),
            Self::ONE_DAY => serializer.serialize_i32(24),
            Self::TWO_DAYS => serializer.serialize_i32(48),
            Self::ONE_WEEK => serializer.serialize_i32(168),
            Self::TWO_WEEKS => serializer.serialize_i32(336),
        }
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleAnomalyDetectionOptionsLearningDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            1 => Self::ONE_HOUR,
            6 => Self::SIX_HOURS,
            12 => Self::TWELVE_HOURS,
            24 => Self::ONE_DAY,
            48 => Self::TWO_DAYS,
            168 => Self::ONE_WEEK,
            336 => Self::TWO_WEEKS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::Number(s.into()),
            }),
        })
    }
}
