// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType {
    STRING,
    INTEGER,
    FLOAT,
    BOOLEAN,
    DATE,
    TIMESTAMP,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType {
    fn to_string(&self) -> String {
        match self {
            Self::STRING => String::from("STRING"),
            Self::INTEGER => String::from("INTEGER"),
            Self::FLOAT => String::from("FLOAT"),
            Self::BOOLEAN => String::from("BOOLEAN"),
            Self::DATE => String::from("DATE"),
            Self::TIMESTAMP => String::from("TIMESTAMP"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType {
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

impl<'de> Deserialize<'de>
    for ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "STRING" => Self::STRING,
            "INTEGER" => Self::INTEGER,
            "FLOAT" => Self::FLOAT,
            "BOOLEAN" => Self::BOOLEAN,
            "DATE" => Self::DATE,
            "TIMESTAMP" => Self::TIMESTAMP,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
