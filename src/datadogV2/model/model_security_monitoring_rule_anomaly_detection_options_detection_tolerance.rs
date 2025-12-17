// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleAnomalyDetectionOptionsDetectionTolerance {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl Serialize for SecurityMonitoringRuleAnomalyDetectionOptionsDetectionTolerance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            Self::ONE => serializer.serialize_i32(1),
            Self::TWO => serializer.serialize_i32(2),
            Self::THREE => serializer.serialize_i32(3),
            Self::FOUR => serializer.serialize_i32(4),
            Self::FIVE => serializer.serialize_i32(5),
        }
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleAnomalyDetectionOptionsDetectionTolerance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            1 => Self::ONE,
            2 => Self::TWO,
            3 => Self::THREE,
            4 => Self::FOUR,
            5 => Self::FIVE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::Number(s.into()),
            }),
        })
    }
}
