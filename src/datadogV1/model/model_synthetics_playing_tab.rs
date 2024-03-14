// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsPlayingTab {
    MAIN_TAB,
    NEW_TAB,
    TAB_1,
    TAB_2,
    TAB_3,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl Serialize for SyntheticsPlayingTab {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            Self::MAIN_TAB => serializer.serialize_i64(-1),
            Self::NEW_TAB => serializer.serialize_i64(0),
            Self::TAB_1 => serializer.serialize_i64(1),
            Self::TAB_2 => serializer.serialize_i64(2),
            Self::TAB_3 => serializer.serialize_i64(3),
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsPlayingTab {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i64 = i64::deserialize(deserializer)?;
        Ok(match s {
            -1 => Self::MAIN_TAB,
            0 => Self::NEW_TAB,
            1 => Self::TAB_1,
            2 => Self::TAB_2,
            3 => Self::TAB_3,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::Number(s.into()),
            }),
        })
    }
}
