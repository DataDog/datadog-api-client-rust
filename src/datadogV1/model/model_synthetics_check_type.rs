// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsCheckType {
    EQUALS,
    NOT_EQUALS,
    CONTAINS,
    NOT_CONTAINS,
    STARTS_WITH,
    NOT_STARTS_WITH,
    GREATER,
    LOWER,
    GREATER_EQUALS,
    LOWER_EQUALS,
    MATCH_REGEX,
    BETWEEN,
    IS_EMPTY,
    NOT_IS_EMPTY,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for SyntheticsCheckType {
    fn to_string(&self) -> String {
        match self {
            Self::EQUALS => String::from("equals"),
            Self::NOT_EQUALS => String::from("notEquals"),
            Self::CONTAINS => String::from("contains"),
            Self::NOT_CONTAINS => String::from("notContains"),
            Self::STARTS_WITH => String::from("startsWith"),
            Self::NOT_STARTS_WITH => String::from("notStartsWith"),
            Self::GREATER => String::from("greater"),
            Self::LOWER => String::from("lower"),
            Self::GREATER_EQUALS => String::from("greaterEquals"),
            Self::LOWER_EQUALS => String::from("lowerEquals"),
            Self::MATCH_REGEX => String::from("matchRegex"),
            Self::BETWEEN => String::from("between"),
            Self::IS_EMPTY => String::from("isEmpty"),
            Self::NOT_IS_EMPTY => String::from("notIsEmpty"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsCheckType {
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

impl<'de> Deserialize<'de> for SyntheticsCheckType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "equals" => Self::EQUALS,
            "notEquals" => Self::NOT_EQUALS,
            "contains" => Self::CONTAINS,
            "notContains" => Self::NOT_CONTAINS,
            "startsWith" => Self::STARTS_WITH,
            "notStartsWith" => Self::NOT_STARTS_WITH,
            "greater" => Self::GREATER,
            "lower" => Self::LOWER,
            "greaterEquals" => Self::GREATER_EQUALS,
            "lowerEquals" => Self::LOWER_EQUALS,
            "matchRegex" => Self::MATCH_REGEX,
            "between" => Self::BETWEEN,
            "isEmpty" => Self::IS_EMPTY,
            "notIsEmpty" => Self::NOT_IS_EMPTY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
