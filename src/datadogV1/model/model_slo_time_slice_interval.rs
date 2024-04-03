// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SLOTimeSliceInterval {
    ONE_MINUTE,
    FIVE_MINUTES,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl Serialize for SLOTimeSliceInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            Self::ONE_MINUTE => serializer.serialize_i32(60),
            Self::FIVE_MINUTES => serializer.serialize_i32(300),
        }
    }
}

impl<'de> Deserialize<'de> for SLOTimeSliceInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            60 => Self::ONE_MINUTE,
            300 => Self::FIVE_MINUTES,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::Number(s.into()),
            }),
        })
    }
}
