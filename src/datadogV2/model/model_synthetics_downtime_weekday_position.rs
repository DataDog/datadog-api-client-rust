// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsDowntimeWeekdayPosition {
    FIRST,
    SECOND,
    THIRD,
    FOURTH,
    LAST,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl Serialize for SyntheticsDowntimeWeekdayPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            Self::FIRST => serializer.serialize_i64(1),
            Self::SECOND => serializer.serialize_i64(2),
            Self::THIRD => serializer.serialize_i64(3),
            Self::FOURTH => serializer.serialize_i64(4),
            Self::LAST => serializer.serialize_i64(-1),
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsDowntimeWeekdayPosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i64 = i64::deserialize(deserializer)?;
        Ok(match s {
            1 => Self::FIRST,
            2 => Self::SECOND,
            3 => Self::THIRD,
            4 => Self::FOURTH,
            -1 => Self::LAST,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::Number(s.into()),
            }),
        })
    }
}
