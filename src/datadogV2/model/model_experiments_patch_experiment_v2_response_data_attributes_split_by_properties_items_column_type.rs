// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsPatchExperimentV2ResponseDataAttributesSplitByPropertiesItemsColumnType {
    VARCHAR,
    INT,
    DOUBLE,
    BOOLEAN_DATADOG,
    VARCHAR_ARRAY,
    INT_ARRAY,
    DOUBLE_ARRAY,
    RAW,
    STRING,
    INTEGER,
    FLOAT,
    BOOLEAN_WAREHOUSE,
    DATE,
    TIMESTAMP,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString
    for ExperimentsPatchExperimentV2ResponseDataAttributesSplitByPropertiesItemsColumnType
{
    fn to_string(&self) -> String {
        match self {
            Self::VARCHAR => String::from("varchar"),
            Self::INT => String::from("int"),
            Self::DOUBLE => String::from("double"),
            Self::BOOLEAN_DATADOG => String::from("boolean"),
            Self::VARCHAR_ARRAY => String::from("varchar_array"),
            Self::INT_ARRAY => String::from("int_array"),
            Self::DOUBLE_ARRAY => String::from("double_array"),
            Self::RAW => String::from("raw"),
            Self::STRING => String::from("STRING"),
            Self::INTEGER => String::from("INTEGER"),
            Self::FLOAT => String::from("FLOAT"),
            Self::BOOLEAN_WAREHOUSE => String::from("BOOLEAN"),
            Self::DATE => String::from("DATE"),
            Self::TIMESTAMP => String::from("TIMESTAMP"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize
    for ExperimentsPatchExperimentV2ResponseDataAttributesSplitByPropertiesItemsColumnType
{
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
    for ExperimentsPatchExperimentV2ResponseDataAttributesSplitByPropertiesItemsColumnType
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "varchar" => Self::VARCHAR,
            "int" => Self::INT,
            "double" => Self::DOUBLE,
            "boolean" => Self::BOOLEAN_DATADOG,
            "varchar_array" => Self::VARCHAR_ARRAY,
            "int_array" => Self::INT_ARRAY,
            "double_array" => Self::DOUBLE_ARRAY,
            "raw" => Self::RAW,
            "STRING" => Self::STRING,
            "INTEGER" => Self::INTEGER,
            "FLOAT" => Self::FLOAT,
            "BOOLEAN" => Self::BOOLEAN_WAREHOUSE,
            "DATE" => Self::DATE,
            "TIMESTAMP" => Self::TIMESTAMP,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
