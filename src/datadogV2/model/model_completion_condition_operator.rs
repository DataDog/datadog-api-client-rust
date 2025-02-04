// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompletionConditionOperator {
    OPERATOR_EQUAL,
    OPERATOR_NOT_EQUAL,
    OPERATOR_GREATER_THAN,
    OPERATOR_LESS_THAN,
    OPERATOR_GREATER_THAN_OR_EQUAL_TO,
    OPERATOR_LESS_THAN_OR_EQUAL_TO,
    OPERATOR_CONTAINS,
    OPERATOR_DOES_NOT_CONTAIN,
    OPERATOR_IS_NULL,
    OPERATOR_IS_NOT_NULL,
    OPERATOR_IS_EMPTY,
    OPERATOR_IS_NOT_EMPTY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CompletionConditionOperator {
    fn to_string(&self) -> String {
        match self {
            Self::OPERATOR_EQUAL => String::from("OPERATOR_EQUAL"),
            Self::OPERATOR_NOT_EQUAL => String::from("OPERATOR_NOT_EQUAL"),
            Self::OPERATOR_GREATER_THAN => String::from("OPERATOR_GREATER_THAN"),
            Self::OPERATOR_LESS_THAN => String::from("OPERATOR_LESS_THAN"),
            Self::OPERATOR_GREATER_THAN_OR_EQUAL_TO => {
                String::from("OPERATOR_GREATER_THAN_OR_EQUAL_TO")
            }
            Self::OPERATOR_LESS_THAN_OR_EQUAL_TO => String::from("OPERATOR_LESS_THAN_OR_EQUAL_TO"),
            Self::OPERATOR_CONTAINS => String::from("OPERATOR_CONTAINS"),
            Self::OPERATOR_DOES_NOT_CONTAIN => String::from("OPERATOR_DOES_NOT_CONTAIN"),
            Self::OPERATOR_IS_NULL => String::from("OPERATOR_IS_NULL"),
            Self::OPERATOR_IS_NOT_NULL => String::from("OPERATOR_IS_NOT_NULL"),
            Self::OPERATOR_IS_EMPTY => String::from("OPERATOR_IS_EMPTY"),
            Self::OPERATOR_IS_NOT_EMPTY => String::from("OPERATOR_IS_NOT_EMPTY"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CompletionConditionOperator {
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

impl<'de> Deserialize<'de> for CompletionConditionOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "OPERATOR_EQUAL" => Self::OPERATOR_EQUAL,
            "OPERATOR_NOT_EQUAL" => Self::OPERATOR_NOT_EQUAL,
            "OPERATOR_GREATER_THAN" => Self::OPERATOR_GREATER_THAN,
            "OPERATOR_LESS_THAN" => Self::OPERATOR_LESS_THAN,
            "OPERATOR_GREATER_THAN_OR_EQUAL_TO" => Self::OPERATOR_GREATER_THAN_OR_EQUAL_TO,
            "OPERATOR_LESS_THAN_OR_EQUAL_TO" => Self::OPERATOR_LESS_THAN_OR_EQUAL_TO,
            "OPERATOR_CONTAINS" => Self::OPERATOR_CONTAINS,
            "OPERATOR_DOES_NOT_CONTAIN" => Self::OPERATOR_DOES_NOT_CONTAIN,
            "OPERATOR_IS_NULL" => Self::OPERATOR_IS_NULL,
            "OPERATOR_IS_NOT_NULL" => Self::OPERATOR_IS_NOT_NULL,
            "OPERATOR_IS_EMPTY" => Self::OPERATOR_IS_EMPTY,
            "OPERATOR_IS_NOT_EMPTY" => Self::OPERATOR_IS_NOT_EMPTY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
