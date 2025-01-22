// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpecVersion {
    ONE_ZERO,
    ONE_ONE,
    ONE_TWO,
    ONE_THREE,
    ONE_FOUR,
    ONE_FIVE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SpecVersion {
    fn to_string(&self) -> String {
        match self {
            Self::ONE_ZERO => String::from("1.0"),
            Self::ONE_ONE => String::from("1.1"),
            Self::ONE_TWO => String::from("1.2"),
            Self::ONE_THREE => String::from("1.3"),
            Self::ONE_FOUR => String::from("1.4"),
            Self::ONE_FIVE => String::from("1.5"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SpecVersion {
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

impl<'de> Deserialize<'de> for SpecVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "1.0" => Self::ONE_ZERO,
            "1.1" => Self::ONE_ONE,
            "1.2" => Self::ONE_TWO,
            "1.3" => Self::ONE_THREE,
            "1.4" => Self::ONE_FOUR,
            "1.5" => Self::ONE_FIVE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
