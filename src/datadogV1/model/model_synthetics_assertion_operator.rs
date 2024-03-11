// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsAssertionOperator {
    CONTAINS,
    DOES_NOT_CONTAIN,
    IS,
    IS_NOT,
    LESS_THAN,
    LESS_THAN_OR_EQUAL,
    MORE_THAN,
    MORE_THAN_OR_EQUAL,
    MATCHES,
    DOES_NOT_MATCH,
    VALIDATES,
    IS_IN_MORE_DAYS_THAN,
    IS_IN_LESS_DAYS_THAN,
    DOES_NOT_EXIST,
    IS_UNDEFINED,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for SyntheticsAssertionOperator {
    fn to_string(&self) -> String {
        match self {
            Self::CONTAINS => String::from("contains"),
            Self::DOES_NOT_CONTAIN => String::from("doesNotContain"),
            Self::IS => String::from("is"),
            Self::IS_NOT => String::from("isNot"),
            Self::LESS_THAN => String::from("lessThan"),
            Self::LESS_THAN_OR_EQUAL => String::from("lessThanOrEqual"),
            Self::MORE_THAN => String::from("moreThan"),
            Self::MORE_THAN_OR_EQUAL => String::from("moreThanOrEqual"),
            Self::MATCHES => String::from("matches"),
            Self::DOES_NOT_MATCH => String::from("doesNotMatch"),
            Self::VALIDATES => String::from("validates"),
            Self::IS_IN_MORE_DAYS_THAN => String::from("isInMoreThan"),
            Self::IS_IN_LESS_DAYS_THAN => String::from("isInLessThan"),
            Self::DOES_NOT_EXIST => String::from("doesNotExist"),
            Self::IS_UNDEFINED => String::from("isUndefined"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsAssertionOperator {
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

impl<'de> Deserialize<'de> for SyntheticsAssertionOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "contains" => Self::CONTAINS,
            "doesNotContain" => Self::DOES_NOT_CONTAIN,
            "is" => Self::IS,
            "isNot" => Self::IS_NOT,
            "lessThan" => Self::LESS_THAN,
            "lessThanOrEqual" => Self::LESS_THAN_OR_EQUAL,
            "moreThan" => Self::MORE_THAN,
            "moreThanOrEqual" => Self::MORE_THAN_OR_EQUAL,
            "matches" => Self::MATCHES,
            "doesNotMatch" => Self::DOES_NOT_MATCH,
            "validates" => Self::VALIDATES,
            "isInMoreThan" => Self::IS_IN_MORE_DAYS_THAN,
            "isInLessThan" => Self::IS_IN_LESS_DAYS_THAN,
            "doesNotExist" => Self::DOES_NOT_EXIST,
            "isUndefined" => Self::IS_UNDEFINED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
