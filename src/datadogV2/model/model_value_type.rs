// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValueType {
    BOOLEAN,
    INTEGER,
    NUMERIC,
    STRING,
    JSON,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ValueType {
    fn to_string(&self) -> String {
        match self {
            Self::BOOLEAN => String::from("BOOLEAN"),
            Self::INTEGER => String::from("INTEGER"),
            Self::NUMERIC => String::from("NUMERIC"),
            Self::STRING => String::from("STRING"),
            Self::JSON => String::from("JSON"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ValueType {
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

impl<'de> Deserialize<'de> for ValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BOOLEAN" => Self::BOOLEAN,
            "INTEGER" => Self::INTEGER,
            "NUMERIC" => Self::NUMERIC,
            "STRING" => Self::STRING,
            "JSON" => Self::JSON,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
