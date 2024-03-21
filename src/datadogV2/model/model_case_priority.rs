// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CasePriority {
    NOT_DEFINED,
    P1,
    P2,
    P3,
    P4,
    P5,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CasePriority {
    fn to_string(&self) -> String {
        match self {
            Self::NOT_DEFINED => String::from("NOT_DEFINED"),
            Self::P1 => String::from("P1"),
            Self::P2 => String::from("P2"),
            Self::P3 => String::from("P3"),
            Self::P4 => String::from("P4"),
            Self::P5 => String::from("P5"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CasePriority {
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

impl<'de> Deserialize<'de> for CasePriority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "NOT_DEFINED" => Self::NOT_DEFINED,
            "P1" => Self::P1,
            "P2" => Self::P2,
            "P3" => Self::P3,
            "P4" => Self::P4,
            "P5" => Self::P5,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
