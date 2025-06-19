// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AlertEventCustomAttributesPriority {
    PRIORITY_ONE,
    PRIORITY_TWO,
    PRIORITY_THREE,
    PRIORITY_FOUR,
    PRIORITY_FIVE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AlertEventCustomAttributesPriority {
    fn to_string(&self) -> String {
        match self {
            Self::PRIORITY_ONE => String::from("1"),
            Self::PRIORITY_TWO => String::from("2"),
            Self::PRIORITY_THREE => String::from("3"),
            Self::PRIORITY_FOUR => String::from("4"),
            Self::PRIORITY_FIVE => String::from("5"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AlertEventCustomAttributesPriority {
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

impl<'de> Deserialize<'de> for AlertEventCustomAttributesPriority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "1" => Self::PRIORITY_ONE,
            "2" => Self::PRIORITY_TWO,
            "3" => Self::PRIORITY_THREE,
            "4" => Self::PRIORITY_FOUR,
            "5" => Self::PRIORITY_FIVE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
