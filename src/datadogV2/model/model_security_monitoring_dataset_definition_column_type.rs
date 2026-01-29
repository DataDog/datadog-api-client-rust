// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringDatasetDefinitionColumnType {
    STRING,
    INTEGER,
    DOUBLE,
    BOOLEAN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringDatasetDefinitionColumnType {
    fn to_string(&self) -> String {
        match self {
            Self::STRING => String::from("string"),
            Self::INTEGER => String::from("integer"),
            Self::DOUBLE => String::from("double"),
            Self::BOOLEAN => String::from("boolean"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringDatasetDefinitionColumnType {
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

impl<'de> Deserialize<'de> for SecurityMonitoringDatasetDefinitionColumnType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "string" => Self::STRING,
            "integer" => Self::INTEGER,
            "double" => Self::DOUBLE,
            "boolean" => Self::BOOLEAN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
