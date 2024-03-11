// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetricIntakeType {
    UNSPECIFIED,
    COUNT,
    RATE,
    GAUGE,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl Serialize for MetricIntakeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            Self::UNSPECIFIED => serializer.serialize_i32(0),
            Self::COUNT => serializer.serialize_i32(1),
            Self::RATE => serializer.serialize_i32(2),
            Self::GAUGE => serializer.serialize_i32(3),
        }
    }
}

impl<'de> Deserialize<'de> for MetricIntakeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => Self::UNSPECIFIED,
            1 => Self::COUNT,
            2 => Self::RATE,
            3 => Self::GAUGE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::Number(s.into()),
            }),
        })
    }
}
