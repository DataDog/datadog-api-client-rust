// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutputSchemaParametersType {
    STRING,
    NUMBER,
    BOOLEAN,
    OBJECT,
    ARRAY_STRING,
    ARRAY_NUMBER,
    ARRAY_BOOLEAN,
    ARRAY_OBJECT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OutputSchemaParametersType {
    fn to_string(&self) -> String {
        match self {
            Self::STRING => String::from("STRING"),
            Self::NUMBER => String::from("NUMBER"),
            Self::BOOLEAN => String::from("BOOLEAN"),
            Self::OBJECT => String::from("OBJECT"),
            Self::ARRAY_STRING => String::from("ARRAY_STRING"),
            Self::ARRAY_NUMBER => String::from("ARRAY_NUMBER"),
            Self::ARRAY_BOOLEAN => String::from("ARRAY_BOOLEAN"),
            Self::ARRAY_OBJECT => String::from("ARRAY_OBJECT"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OutputSchemaParametersType {
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

impl<'de> Deserialize<'de> for OutputSchemaParametersType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "STRING" => Self::STRING,
            "NUMBER" => Self::NUMBER,
            "BOOLEAN" => Self::BOOLEAN,
            "OBJECT" => Self::OBJECT,
            "ARRAY_STRING" => Self::ARRAY_STRING,
            "ARRAY_NUMBER" => Self::ARRAY_NUMBER,
            "ARRAY_BOOLEAN" => Self::ARRAY_BOOLEAN,
            "ARRAY_OBJECT" => Self::ARRAY_OBJECT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
