// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsPatchTestOperationName {
    ADD,
    REMOVE,
    REPLACE,
    MOVE,
    COPY,
    TEST,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for SyntheticsPatchTestOperationName {
    fn to_string(&self) -> String {
        match self {
            Self::ADD => String::from("add"),
            Self::REMOVE => String::from("remove"),
            Self::REPLACE => String::from("replace"),
            Self::MOVE => String::from("move"),
            Self::COPY => String::from("copy"),
            Self::TEST => String::from("test"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsPatchTestOperationName {
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

impl<'de> Deserialize<'de> for SyntheticsPatchTestOperationName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "add" => Self::ADD,
            "remove" => Self::REMOVE,
            "replace" => Self::REPLACE,
            "move" => Self::MOVE,
            "copy" => Self::COPY,
            "test" => Self::TEST,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
