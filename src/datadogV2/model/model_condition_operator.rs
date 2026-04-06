// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConditionOperator {
    LT,
    LTE,
    GT,
    GTE,
    MATCHES,
    NOT_MATCHES,
    ONE_OF,
    NOT_ONE_OF,
    IS_NULL,
    EQUALS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ConditionOperator {
    fn to_string(&self) -> String {
        match self {
            Self::LT => String::from("LT"),
            Self::LTE => String::from("LTE"),
            Self::GT => String::from("GT"),
            Self::GTE => String::from("GTE"),
            Self::MATCHES => String::from("MATCHES"),
            Self::NOT_MATCHES => String::from("NOT_MATCHES"),
            Self::ONE_OF => String::from("ONE_OF"),
            Self::NOT_ONE_OF => String::from("NOT_ONE_OF"),
            Self::IS_NULL => String::from("IS_NULL"),
            Self::EQUALS => String::from("EQUALS"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ConditionOperator {
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

impl<'de> Deserialize<'de> for ConditionOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "LT" => Self::LT,
            "LTE" => Self::LTE,
            "GT" => Self::GT,
            "GTE" => Self::GTE,
            "MATCHES" => Self::MATCHES,
            "NOT_MATCHES" => Self::NOT_MATCHES,
            "ONE_OF" => Self::ONE_OF,
            "NOT_ONE_OF" => Self::NOT_ONE_OF,
            "IS_NULL" => Self::IS_NULL,
            "EQUALS" => Self::EQUALS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
